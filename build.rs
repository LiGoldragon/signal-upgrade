use protos::WireContractFamily;
use schema_rust::build::{ContractCrateBuild, CrateName, SchemaVersion, UpdateEnvironmentVariable};

fn main() {
    ContractCrateBuild::from_environment(
        CrateName::new("signal-upgrade"),
        SchemaVersion::new("0.2.3"),
        UpdateEnvironmentVariable::new("SIGNAL_UPGRADE_UPDATE_SCHEMA_ARTIFACTS"),
        WireContractFamily::SignalSpirit,
    )
    .expect_fresh();
}
