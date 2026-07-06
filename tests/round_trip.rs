#[cfg(feature = "nota-text")]
use nota::{NotaDecode, NotaEncode, NotaSource};
use signal_frame::{
    ExchangeIdentifier, ExchangeLane, LaneSequence, Reply as FrameReply, SessionEpoch,
    SignalOperationHeads, SubReply,
};
use signal_upgrade::{
    Attempt, Completion, CompletionReport, ComponentName, ContractVersion, Date, DivergencePayload,
    DivergenceReason, Frame, FrameBody, HandoverMarker, HandoverRejection, HandoverRejectionReason,
    Input, InputRoute, Inspection, MirrorAcknowledgement, MirrorPayload, Output, OutputRoute,
    RawByte, RawBytes, ReadinessReport, RecordKind, RecoveryRequest, RecoveryResult, Rejection,
    RejectionReason, ReportQuery, Reported, SupportedMigration, Time, UnimplementedReason, Version,
};

#[cfg(feature = "nota-text")]
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
    Version {
        major: 0,
        minor: 1,
        patch: 0,
    }
}

fn target() -> Version {
    Version {
        major: 0,
        minor: 1,
        patch: 1,
    }
}

fn migration() -> String {
    String::from("persona-spirit-0-1-0-to-0-1-1")
}

fn supported_migration() -> SupportedMigration {
    SupportedMigration {
        component_name: component(),
        source: source(),
        target: target(),
        migration_identifier: migration().into(),
    }
}

fn attempt() -> Attempt {
    Attempt {
        component_name: component(),
        source: source(),
        target: target(),
    }
}

fn completion() -> Completion {
    Completion {
        component_name: component(),
        source: source(),
        target: target(),
        migration_identifier: migration().into(),
        changed_records: 103.into(),
    }
}

fn contract_version(byte: u64) -> ContractVersion {
    ContractVersion::new(raw_bytes(vec![byte; 32]))
}

fn raw_bytes(bytes: Vec<u64>) -> RawBytes {
    RawBytes::new(bytes.into_iter().map(RawByte::new).collect())
}

fn record_kind() -> RecordKind {
    RecordKind::new("Entry")
}

fn marker() -> HandoverMarker {
    HandoverMarker {
        component_name: component(),
        contract_version: contract_version(1),
        state_sequence: 34.into(),
        mirrored_write_count: 55.into(),
        record_frontier: Some(103).into(),
        date: Date {
            year: 2026,
            month: 5,
            day: 22,
        },
        time: Time {
            hour: 11,
            minute: 42,
            second: 0,
        },
    }
}

fn mirror_payload() -> MirrorPayload {
    MirrorPayload {
        component_name: component(),
        source_version: contract_version(1),
        target_version: contract_version(2),
        record_kind: record_kind(),
        raw_bytes: raw_bytes(vec![1, 2, 3]),
    }
}

fn round_trip_input(input: Input) -> Input {
    let frame = input.clone().into_frame(exchange());
    let bytes = frame.encode_length_prefixed().expect("encode");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode");
    match decoded.into_body() {
        FrameBody::Request { request, .. } => request.payloads().head().clone(),
        other => panic!("expected request frame, got {other:?}"),
    }
}

fn round_trip_output(output: Output) -> Output {
    let frame = output.clone().into_reply_frame(exchange());
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

#[cfg(feature = "nota-text")]
fn encode<T: NotaEncode>(value: &T) -> String {
    value.to_nota()
}

#[cfg(feature = "nota-text")]
fn round_trip_nota<T>(value: T, expected: &str)
where
    T: NotaEncode + NotaDecode + PartialEq + std::fmt::Debug,
{
    let encoded = encode(&value);
    assert_eq!(encoded, expected);

    let recovered = NotaSource::new(&encoded).parse::<T>().expect("decode nota");
    assert_eq!(recovered, value);
    assert!(
        CANONICAL.contains(expected),
        "examples/canonical.nota missing line: {expected}"
    );
}

#[test]
fn catalogue_requests_round_trip_through_signal_frames() {
    let inputs = [
        Input::inspect(Inspection::All),
        Input::inspect(Inspection::component(component().into_payload())),
        Input::attempt_upgrade(attempt()),
        Input::report(ReportQuery::component(component().into_payload())),
    ];

    for input in inputs {
        assert_eq!(round_trip_input(input.clone()), input);
    }
}

#[test]
fn handover_requests_round_trip_through_signal_frames() {
    let inputs = [
        Input::ask_handover_marker(component()),
        Input::ready_to_handover(ReadinessReport {
            component_name: component(),
            handover_marker: marker(),
        }),
        Input::handover_completed(CompletionReport {
            component_name: component(),
            handover_marker: marker(),
        }),
        Input::mirror(mirror_payload()),
        Input::divergence(DivergencePayload {
            component_name: component(),
            source_version: contract_version(1),
            target_version: contract_version(2),
            divergence_reason: DivergenceReason::NotRepresentable,
            record_kind: record_kind(),
            raw_bytes: raw_bytes(vec![9, 8, 7]),
        }),
        Input::recover_from_failure(RecoveryRequest {
            component_name: component(),
            failure_identifier: 7.into(),
        }),
    ];

    for input in inputs {
        assert_eq!(round_trip_input(input.clone()), input);
    }
}

#[test]
fn replies_round_trip_through_signal_frames() {
    let outputs = [
        Output::inspection_reported(vec![supported_migration()].into()),
        Output::upgrade_completed(completion()),
        Output::upgrade_rejected(Rejection {
            component_name: component(),
            source: source(),
            target: Version {
                major: 0,
                minor: 1,
                patch: 2,
            },
            rejection_reason: RejectionReason::UnsupportedMigration,
        }),
        Output::reported(Reported {
            completions: vec![completion()].into(),
            rejections: vec![].into(),
        }),
        Output::handover_marker(marker()),
        Output::handover_accepted(marker()),
        Output::handover_finalized(marker()),
        Output::mirror_acknowledged(MirrorAcknowledgement {
            component_name: component(),
            mirrored_write_count: 56.into(),
        }),
        Output::recovery_completed(RecoveryResult {
            component_name: component(),
            recovered: true.into(),
        }),
        Output::handover_rejected(HandoverRejection {
            component_name: component(),
            handover_rejection_reason: HandoverRejectionReason::StateSequenceAdvanced,
        }),
        Output::request_unimplemented(UnimplementedReason::NotBuiltYet),
    ];

    for output in outputs {
        assert_eq!(round_trip_output(output.clone()), output);
    }
}

#[test]
fn contract_owned_routes_are_generated_for_both_surfaces() {
    assert_eq!(Input::inspect(Inspection::All).route(), InputRoute::Inspect);
    assert_eq!(
        Input::attempt_upgrade(attempt()).route(),
        InputRoute::AttemptUpgrade
    );
    assert_eq!(Input::mirror(mirror_payload()).route(), InputRoute::Mirror);
    assert_eq!(
        Input::recover_from_failure(RecoveryRequest {
            component_name: component(),
            failure_identifier: 7.into(),
        })
        .route(),
        InputRoute::RecoverFromFailure
    );
    assert_eq!(
        Output::request_unimplemented(UnimplementedReason::NotBuiltYet).route(),
        OutputRoute::RequestUnimplemented
    );
}

#[test]
fn generated_wire_contract_exposes_signal_frame_request_heads() {
    assert!(<Input as SignalOperationHeads>::contains_head("Inspect"));
    assert!(<Input as SignalOperationHeads>::contains_head(
        "AttemptUpgrade"
    ));
    assert!(<Input as SignalOperationHeads>::contains_head("Mirror"));
    assert!(!<Input as SignalOperationHeads>::contains_head(
        "CommandSemaWrite"
    ));
}

#[test]
#[cfg(feature = "nota-text")]
fn canonical_catalogue_nota_examples_round_trip() {
    round_trip_nota(Input::inspect(Inspection::All), "(Inspect All)");
    round_trip_nota(
        Input::inspect(Inspection::component(component().into_payload())),
        "(Inspect (Component persona-spirit))",
    );
    round_trip_nota(
        Input::attempt_upgrade(attempt()),
        "(AttemptUpgrade (persona-spirit (0 1 0) (0 1 1)))",
    );
    round_trip_nota(
        Input::report(ReportQuery::component(component().into_payload())),
        "(Report (Component persona-spirit))",
    );
    round_trip_nota(
        Output::inspection_reported(vec![supported_migration()].into()),
        "(InspectionReported [(persona-spirit (0 1 0) (0 1 1) persona-spirit-0-1-0-to-0-1-1)])",
    );
    round_trip_nota(
        Output::upgrade_completed(completion()),
        "(UpgradeCompleted (persona-spirit (0 1 0) (0 1 1) persona-spirit-0-1-0-to-0-1-1 103))",
    );
    round_trip_nota(
        Output::upgrade_rejected(Rejection {
            component_name: component(),
            source: source(),
            target: Version {
                major: 0,
                minor: 1,
                patch: 2,
            },
            rejection_reason: RejectionReason::UnsupportedMigration,
        }),
        "(UpgradeRejected (persona-spirit (0 1 0) (0 1 2) UnsupportedMigration))",
    );
    round_trip_nota(
        Output::request_unimplemented(UnimplementedReason::NotBuiltYet),
        "(RequestUnimplemented NotBuiltYet)",
    );
}

#[test]
#[cfg(feature = "nota-text")]
fn handover_marker_reply_round_trips_through_nota() {
    let output = Output::handover_marker(marker());
    let text = encode(&output);

    assert!(text.starts_with("(HandoverMarker (persona-spirit "));

    let decoded = NotaSource::new(&text).parse::<Output>().expect("decode");
    assert_eq!(decoded, output);
}

#[test]
fn mirror_payload_carries_source_target_kind_and_raw_bytes() {
    let payload = mirror_payload();
    let input = Input::mirror(payload.clone());

    assert_eq!(input.route(), InputRoute::Mirror);
    assert_eq!(input, Input::mirror(payload));
}

#[test]
#[cfg(feature = "nota-text")]
fn divergence_reason_encodes_as_unit_variant() {
    let reason = DivergenceReason::NotRepresentable;
    assert_eq!(encode(&reason), "NotRepresentable");
}
