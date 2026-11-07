# Upgrades

## 5.0.0 — repin to the 0.33 cohort

- Pins: datom-codec eabead4 (0.33.0), protos 3c7ade3 (0.33.0), build-dependency ethos-zero
  5a5daca (19.2.0). One revision of each in the lock.
- Rust API only: `src/generated/signal.rs` is regenerated with ethos-zero 19.2.0, so every
  scalar alias is now a tuple struct. A consumer wraps a value with the constructor and reads it
  as `.0`; `RequestUnimplementedPayload` is built as
  `RequestUnimplementedPayload(UnimplementedReason::NotBuiltYet)`.
- `ReaderBudget` fields are private: build one with `ReaderBudget::try_from(n).unwrap()`.
- The contract holds no `Decimal`.
