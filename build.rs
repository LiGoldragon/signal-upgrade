use schema_rust_next::build::ContractCrateBuild;

fn main() {
    ContractCrateBuild::from_environment(
        "signal-upgrade",
        "0.2.0",
        "SIGNAL_UPGRADE_UPDATE_SCHEMA_ARTIFACTS",
    )
    .expect_fresh();
}
