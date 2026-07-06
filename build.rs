use schema_rust::build::ContractCrateBuild;

fn main() {
    ContractCrateBuild::from_environment(
        "signal-upgrade",
        "0.2.3",
        "SIGNAL_UPGRADE_UPDATE_SCHEMA_ARTIFACTS",
    )
    .expect_fresh();
}
