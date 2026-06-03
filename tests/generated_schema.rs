use signal_upgrade::schema::lib::{
    Attempt, Completion, ComponentName, Input, InputRoute, NexusAction, NexusActionRoute,
    NexusWork, ObjectName, OriginRoute, Output, OutputRoute, SemaWriteInput, SemaWriteInputRoute,
    SemaWriteOutput, SignalObjectName, TraceEvent, Version,
};

fn version(major: u64, minor: u64, patch: u64) -> Version {
    Version {
        major,
        minor,
        patch,
    }
}

fn attempt() -> Attempt {
    Attempt {
        component: ComponentName::from("persona-spirit"),
        source: version(0, 1, 0),
        target: version(0, 1, 1),
    }
}

fn completion() -> Completion {
    Completion {
        component: ComponentName::from("persona-spirit"),
        source: version(0, 1, 0),
        target: version(0, 1, 1),
        migration: String::from("persona-spirit-0-1-0-to-0-1-1"),
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
fn generated_signal_nexus_sema_projection_routes_attempt_upgrade() {
    let work = NexusWork::signal_arrived(Input::attempt_upgrade(attempt()))
        .with_origin_route(OriginRoute(17));
    let action = work.into_nexus_action();

    assert_eq!(action.origin_route(), OriginRoute(17));
    assert_eq!(action.root().route(), NexusActionRoute::CommandSemaWrite);
    match action.root() {
        NexusAction::CommandSemaWrite(SemaWriteInput::AttemptUpgrade(payload)) => {
            assert_eq!(payload.component, "persona-spirit");
        }
        other => panic!("expected AttemptUpgrade SEMA write, got {other:?}"),
    }

    let sema_input = action.into_sema_write_input();
    assert_eq!(sema_input.origin_route(), OriginRoute(17));
    assert_eq!(
        sema_input.root().route(),
        SemaWriteInputRoute::AttemptUpgrade
    );
}

#[test]
fn generated_sema_completion_projects_back_to_signal_output() {
    let output = SemaWriteOutput::upgrade_completed(completion())
        .with_origin_route(OriginRoute(29))
        .into_nexus_work()
        .into_nexus_action()
        .into_signal_output();

    assert_eq!(output.origin_route(), OriginRoute(29));
    assert_eq!(output.root().route(), OutputRoute::UpgradeCompleted);
    match output.into_root() {
        Output::UpgradeCompleted(completion) => {
            assert_eq!(completion.changed_records, 3);
        }
        other => panic!("expected UpgradeCompleted output, got {other:?}"),
    }
}

#[test]
fn generated_trace_vocabulary_names_signal_operation() {
    let trace = TraceEvent::new(ObjectName::Signal(SignalObjectName::Input(
        InputRoute::AttemptUpgrade,
    )));

    assert_eq!(trace.name(), "SignalInputAttemptUpgrade");
}
