use signal_upgrade::{
    ByteViewable, Inspection, Query, RequestUnimplementedPayload, Response, Restorable, Signal,
    Signalizable,
};
#[test]
fn inspect_round_trips_as_fresh_signal() {
    let query = Query::Inspect(Inspection::All);
    let received = Signal::<Query>::from(query.signalize().expect("signalize").bytes().to_vec());
    assert_eq!(received.restore().expect("restore"), query);
}
#[test]
fn rejection_response_round_trips_as_fresh_signal() {
    let response = Response::RequestUnimplemented(RequestUnimplementedPayload::NotBuiltYet);
    let received =
        Signal::<Response>::from(response.signalize().expect("signalize").bytes().to_vec());
    assert_eq!(received.restore().expect("restore"), response);
}

#[cfg(feature = "datom")]
#[test]
fn inspect_round_trips_as_datom_text() {
    use datom_codec::{Actualizing, Budget, Datomizable, Potential};
    use protos::{Protosizable, ReaderBudget, Textualizable};
    let query = Query::Inspect(Inspection::All);
    let text = query.clone().datomize(vec![]).protosize().textualize();
    let mut p = Potential::<Query>::from(text);
    assert_eq!(
        p.actualize(&mut Budget {
            remaining: 1024,
            reader: ReaderBudget { remaining: 1024 },
            depth: 0,
            maximum_depth: 1024
        })
        .expect("actualize"),
        query
    );
}
