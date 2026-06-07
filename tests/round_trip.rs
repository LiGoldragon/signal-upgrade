use nota_codec::{Decoder, Encoder, NotaDecode, NotaEncode};
use signal_frame::{
    ExchangeIdentifier, ExchangeLane, LaneSequence, NonEmpty, Reply as FrameReply, RequestPayload,
    SessionEpoch, SubReply,
};
use signal_upgrade::{
    Attempt, Completion, CompletionReport, ComponentName, Date, DivergencePayload,
    DivergenceReason, EffectEmitted, EffectOutcome, Frame, FrameBody, HandoverAcceptance,
    HandoverFinalization, HandoverMarker, HandoverRejection, HandoverRejectionReason, Inspection,
    InspectionReported, MarkerRequest, MigrationIdentifier, MirrorAcknowledgement, MirrorPayload,
    Operation, OperationKind, ReadinessReport, RecoveryRequest, RecoveryResult, Rejection,
    RejectionReason, Reply, ReportQuery, Reported, RequestUnimplemented, SupportedMigration, Time,
    UnimplementedReason, Version,
};
use version_projection::{ComponentName as ProjectionComponentName, ContractVersion, RecordKind};

const CANONICAL: &str = include_str!("../examples/canonical.nota");

fn exchange() -> ExchangeIdentifier {
    ExchangeIdentifier::new(
        SessionEpoch::new(1),
        ExchangeLane::Connector,
        LaneSequence::first(),
    )
}

fn component() -> ComponentName {
    ComponentName::new("persona-spirit")
}

fn source() -> Version {
    Version::new(0, 1, 0)
}

fn target() -> Version {
    Version::new(0, 1, 1)
}

fn migration() -> MigrationIdentifier {
    MigrationIdentifier::new("persona-spirit-0-1-0-to-0-1-1")
}

fn supported_migration() -> SupportedMigration {
    SupportedMigration {
        component: component(),
        source: source(),
        target: target(),
        identifier: migration(),
    }
}

fn attempt() -> Attempt {
    Attempt {
        component: component(),
        source: source(),
        target: target(),
    }
}

fn completion() -> Completion {
    Completion {
        component: component(),
        source: source(),
        target: target(),
        migration: migration(),
        changed_records: 103,
    }
}

fn projection_component() -> ProjectionComponentName {
    ProjectionComponentName::new("persona-spirit")
}

fn contract_version(byte: u8) -> ContractVersion {
    ContractVersion::new([byte; 32])
}

fn marker() -> HandoverMarker {
    HandoverMarker {
        component: projection_component(),
        schema_hash: contract_version(1),
        state_sequence: 34,
        mirrored_write_count: 55,
        record_frontier: Some(103),
        recorded_at_date: Date::new(2026, 5, 22),
        recorded_at_time: Time::new(11, 42, 0),
    }
}

fn mirror_payload() -> MirrorPayload {
    MirrorPayload {
        component: projection_component(),
        source_version: contract_version(1),
        target_version: contract_version(2),
        kind: RecordKind::new("Entry"),
        payload: vec![1, 2, 3],
    }
}

fn round_trip_request(operation: Operation) -> Operation {
    let frame = Frame::new(FrameBody::Request {
        exchange: exchange(),
        request: operation.clone().into_request(),
    });
    let bytes = frame.encode_length_prefixed().expect("encode");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode");
    match decoded.into_body() {
        FrameBody::Request { request, .. } => request.payloads().head().clone(),
        other => panic!("expected request frame, got {other:?}"),
    }
}

fn round_trip_reply(reply: Reply) -> Reply {
    let frame = Frame::new(FrameBody::Reply {
        exchange: exchange(),
        reply: FrameReply::committed(NonEmpty::single(SubReply::Ok(reply.clone()))),
    });
    let bytes = frame.encode_length_prefixed().expect("encode");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode");
    match decoded.into_body() {
        FrameBody::Reply { reply, .. } => match reply {
            FrameReply::Accepted { per_operation, .. } => match per_operation.into_head() {
                SubReply::Ok(payload) => payload,
                other => panic!("expected accepted reply payload, got {other:?}"),
            },
            other => panic!("expected accepted frame reply, got {other:?}"),
        },
        other => panic!("expected reply frame, got {other:?}"),
    }
}

fn encode<T: NotaEncode>(value: &T) -> String {
    let mut encoder = Encoder::new();
    value.encode(&mut encoder).expect("encode nota");
    encoder.into_string()
}

fn round_trip_nota<T>(value: T, expected: &str)
where
    T: NotaEncode + NotaDecode + PartialEq + std::fmt::Debug,
{
    let encoded = encode(&value);
    assert_eq!(encoded, expected);

    let mut decoder = Decoder::new(&encoded);
    let recovered = T::decode(&mut decoder).expect("decode nota");
    assert_eq!(recovered, value);
    assert!(
        CANONICAL.contains(expected),
        "examples/canonical.nota missing line: {expected}"
    );
}

#[test]
fn catalogue_requests_round_trip_through_signal_frames() {
    let operations = [
        Operation::Inspect(Inspection::All),
        Operation::Inspect(Inspection::Component(component())),
        Operation::AttemptUpgrade(attempt()),
        Operation::Report(ReportQuery::Component(component())),
    ];

    for operation in operations {
        assert_eq!(round_trip_request(operation.clone()), operation);
    }
}

#[test]
fn handover_requests_round_trip_through_signal_frames() {
    let operations = [
        Operation::AskHandoverMarker(MarkerRequest {
            component: projection_component(),
        }),
        Operation::ReadyToHandover(ReadinessReport {
            component: projection_component(),
            source_marker: marker(),
        }),
        Operation::HandoverCompleted(CompletionReport {
            component: projection_component(),
            accepted_marker: marker(),
        }),
        Operation::Mirror(mirror_payload()),
        Operation::Divergence(DivergencePayload {
            component: projection_component(),
            source_version: contract_version(1),
            target_version: contract_version(2),
            reason: DivergenceReason::NotRepresentable,
            kind: RecordKind::new("Entry"),
            payload: vec![9, 8, 7],
        }),
        Operation::RecoverFromFailure(RecoveryRequest {
            component: projection_component(),
            failure_identifier: 7,
        }),
    ];

    for operation in operations {
        assert_eq!(round_trip_request(operation.clone()), operation);
    }
}

#[test]
fn replies_round_trip_through_signal_frames() {
    let replies = [
        Reply::InspectionReported(InspectionReported {
            migrations: vec![supported_migration()],
        }),
        Reply::UpgradeCompleted(completion()),
        Reply::UpgradeRejected(Rejection {
            component: component(),
            source: source(),
            target: Version::new(0, 1, 2),
            reason: RejectionReason::UnsupportedMigration,
        }),
        Reply::Reported(Reported {
            completions: vec![completion()],
            rejections: vec![],
        }),
        Reply::HandoverMarker(marker()),
        Reply::HandoverAccepted(HandoverAcceptance {
            accepted_marker: marker(),
        }),
        Reply::HandoverFinalized(HandoverFinalization {
            finalized_marker: marker(),
        }),
        Reply::MirrorAcknowledged(MirrorAcknowledgement {
            component: projection_component(),
            mirrored_write_count: 56,
        }),
        Reply::RecoveryCompleted(RecoveryResult {
            component: projection_component(),
            recovered: true,
        }),
        Reply::HandoverRejected(HandoverRejection {
            component: projection_component(),
            reason: HandoverRejectionReason::StateSequenceAdvanced,
        }),
        Reply::RequestUnimplemented(RequestUnimplemented {
            reason: UnimplementedReason::NotBuiltYet,
        }),
    ];

    for reply in replies {
        assert_eq!(round_trip_reply(reply.clone()), reply);
    }
}

#[test]
fn contract_owned_operation_kind_is_generated_for_both_surfaces() {
    assert_eq!(
        Operation::Inspect(Inspection::All).kind(),
        OperationKind::Inspect
    );
    assert_eq!(
        Operation::AttemptUpgrade(attempt()).kind(),
        OperationKind::AttemptUpgrade
    );
    assert_eq!(
        Operation::Mirror(mirror_payload()).kind(),
        OperationKind::Mirror
    );
    assert_eq!(
        Operation::RecoverFromFailure(RecoveryRequest {
            component: projection_component(),
            failure_identifier: 7,
        })
        .kind(),
        OperationKind::RecoverFromFailure
    );
}

#[test]
fn effect_event_uses_contract_owned_outcome_not_sema_observation() {
    let event = EffectEmitted {
        operation: OperationKind::AttemptUpgrade,
        outcome: EffectOutcome::UpgradeCompleted,
    };

    let encoded = encode(&event);
    assert_eq!(encoded, "(AttemptUpgrade UpgradeCompleted)");
    assert!(!encoded.contains("Sema"));

    let mut decoder = Decoder::new(&encoded);
    let recovered = EffectEmitted::decode(&mut decoder).expect("decode event");
    assert_eq!(recovered, event);
}

#[test]
fn canonical_catalogue_nota_examples_round_trip() {
    round_trip_nota(Operation::Inspect(Inspection::All), "(Inspect All)");
    round_trip_nota(
        Operation::Inspect(Inspection::Component(component())),
        "(Inspect (Component persona-spirit))",
    );
    round_trip_nota(
        Operation::AttemptUpgrade(attempt()),
        "(AttemptUpgrade (persona-spirit (0 1 0) (0 1 1)))",
    );
    round_trip_nota(
        Operation::Report(ReportQuery::Component(component())),
        "(Report (Component persona-spirit))",
    );
    round_trip_nota(
        Reply::InspectionReported(InspectionReported {
            migrations: vec![supported_migration()],
        }),
        "(InspectionReported ([(persona-spirit (0 1 0) (0 1 1) persona-spirit-0-1-0-to-0-1-1)]))",
    );
    round_trip_nota(
        Reply::UpgradeCompleted(completion()),
        "(UpgradeCompleted (persona-spirit (0 1 0) (0 1 1) persona-spirit-0-1-0-to-0-1-1 103))",
    );
    round_trip_nota(
        Reply::UpgradeRejected(Rejection {
            component: component(),
            source: source(),
            target: Version::new(0, 1, 2),
            reason: RejectionReason::UnsupportedMigration,
        }),
        "(UpgradeRejected (persona-spirit (0 1 0) (0 1 2) UnsupportedMigration))",
    );
    round_trip_nota(
        Reply::RequestUnimplemented(RequestUnimplemented {
            reason: UnimplementedReason::NotBuiltYet,
        }),
        "(RequestUnimplemented (NotBuiltYet))",
    );
}

#[test]
fn handover_marker_reply_round_trips_through_nota() {
    let reply = Reply::HandoverMarker(marker());
    let text = encode(&reply);

    assert!(text.starts_with("(HandoverMarker (persona-spirit "));

    let mut decoder = Decoder::new(&text);
    let decoded = Reply::decode(&mut decoder).expect("decode");
    assert_eq!(decoded, reply);
}

#[test]
fn mirror_payload_carries_source_target_kind_and_raw_bytes() {
    let payload = mirror_payload();
    let operation = Operation::Mirror(payload.clone());

    assert_eq!(operation.kind(), OperationKind::Mirror);
    assert_eq!(operation, Operation::Mirror(payload));
}

#[test]
fn divergence_reason_encodes_as_unit_variant() {
    let reason = DivergenceReason::NotRepresentable;
    assert_eq!(encode(&reason), "NotRepresentable");
}
