#[cfg(feature = "dotos-text")]
use dotos::{DotosEncode, DotosSource};
use signal_frame::{ExchangeIdentifier, ExchangeLane, LaneSequence, Reply, SessionEpoch, SubReply};
use signal_upgrade::{ComponentName, Frame, FrameBody, Input, Output, RecoveryResult};

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

#[test]
fn request_and_reply_round_trip_through_typed_frames() {
    let input = Input::ask_handover_marker(component().into());
    let input_frame = input.clone().into_frame(exchange());
    let input_bytes = input_frame
        .encode_length_prefixed()
        .expect("encode input frame");
    let decoded_input = Frame::decode_length_prefixed(&input_bytes).expect("decode input frame");
    match decoded_input.into_body() {
        FrameBody::Request { request, .. } => assert_eq!(request.payloads().head(), &input),
        other => panic!("expected request, got {other:?}"),
    }

    let output = Output::recovery_completed(RecoveryResult {
        component: component().into(),
        recovered: true.into(),
    });
    let output_frame = output.clone().into_reply_frame(exchange());
    let output_bytes = output_frame
        .encode_length_prefixed()
        .expect("encode reply frame");
    let decoded_output = Frame::decode_length_prefixed(&output_bytes).expect("decode reply frame");
    match decoded_output.into_body() {
        FrameBody::Reply { reply, .. } => match reply {
            Reply::Accepted { per_operation, .. } => match per_operation.into_head() {
                SubReply::Ok(recovered) => assert_eq!(recovered, output),
                other => panic!("expected reply payload, got {other:?}"),
            },
            other => panic!("expected accepted reply, got {other:?}"),
        },
        other => panic!("expected reply, got {other:?}"),
    }
}

#[cfg(feature = "dotos-text")]
#[test]
fn dotted_contract_dotos_round_trip() {
    let input = Input::ask_handover_marker(component().into());
    let encoded = input.to_dotos();
    let decoded = DotosSource::new(&encoded)
        .parse::<Input>()
        .expect("decode dotos input");
    assert_eq!(decoded, input);
}
