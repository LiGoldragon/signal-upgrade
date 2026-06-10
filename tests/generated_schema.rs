use signal_upgrade::schema::lib::{
    Attempt, Completion, ComponentName, Input, InputRoute, Output, OutputRoute, Version,
};

const SCHEMA_SOURCE: &str = include_str!("../schema/lib.schema");
const GENERATED_SCHEMA_RUST: &str = include_str!("../src/schema/lib.rs");

fn version(major: u64, minor: u64, patch: u64) -> Version {
    Version {
        major,
        minor,
        patch,
    }
}

fn attempt() -> Attempt {
    Attempt {
        component: ComponentName::new("persona-spirit"),
        source: version(0, 1, 0),
        target: version(0, 1, 1),
    }
}

fn completion() -> Completion {
    Completion {
        component: ComponentName::new("persona-spirit"),
        source: version(0, 1, 0),
        target: version(0, 1, 1),
        migration: String::from("persona-spirit-0-1-0-to-0-1-1").into(),
        changed_records: 3,
    }
}

#[test]
fn generated_signal_input_owns_short_header_and_frame() {
    let input = Input::attempt_upgrade(attempt());

    assert_eq!(input.route(), InputRoute::AttemptUpgrade);

    let frame = input.encode_signal_frame().expect("encode generated input");
    let (route, decoded) = Input::decode_signal_frame(&frame).expect("decode generated input");

    assert_eq!(route, InputRoute::AttemptUpgrade);
    assert_eq!(decoded, input);
}

#[test]
fn generated_signal_output_owns_short_header_and_frame() {
    let output = Output::upgrade_completed(completion());

    assert_eq!(output.route(), OutputRoute::UpgradeCompleted);

    let frame = output
        .encode_signal_frame()
        .expect("encode generated output");
    let (route, decoded) = Output::decode_signal_frame(&frame).expect("decode generated output");

    assert_eq!(route, OutputRoute::UpgradeCompleted);
    match decoded {
        Output::UpgradeCompleted(completion) => {
            assert_eq!(completion.changed_records, 3);
        }
        other => panic!("expected UpgradeCompleted output, got {other:?}"),
    }
}

#[test]
fn generated_contract_surface_excludes_runtime_plane_terms() {
    for term in [
        "NexusWork",
        "NexusAction",
        "CommandSemaWrite",
        "CommandSemaRead",
        "SemaWriteInput",
        "SemaReadInput",
        "SemaWriteOutput",
        "SemaReadOutput",
        "SignalEngine",
        "NexusEngine",
        "SemaEngine",
        "TraceEvent",
        "ObjectName",
        "SignalObjectName",
        "OriginRoute",
        "MessageIdentifier",
        "MessageSent",
        "MessageProcessed",
        "pub struct Signal<Root>",
        "pub struct Nexus<Root>",
        "pub struct Sema<Root>",
        "pub enum Plane",
        "UpgradeFrom",
        "AcceptPrevious",
    ] {
        assert!(
            !SCHEMA_SOURCE.contains(term),
            "contract schema must not declare runtime term {term}"
        );
        assert!(
            !GENERATED_SCHEMA_RUST.contains(term),
            "generated contract module must not export runtime term {term}"
        );
    }
}
