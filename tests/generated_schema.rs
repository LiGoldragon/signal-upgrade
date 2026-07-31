const SCHEMA_SOURCE: &str = include_str!("../schema/lib.schema");
const GENERATED_SCHEMA_RUST: &str = include_str!("../src/schema/lib.rs");

#[test]
fn generated_contract_surface_is_wire_only() {
    for term in [
        "NexusWork",
        "SignalEngine",
        "SemaEngine",
        "UpgradeFrom",
        "AcceptPrevious",
    ] {
        assert!(
            !SCHEMA_SOURCE.contains(term),
            "schema declares runtime term {term}"
        );
        assert!(
            !GENERATED_SCHEMA_RUST.contains(term),
            "generated contract exports runtime term {term}"
        );
    }
}
