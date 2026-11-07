# Upgrades

## 4.1.0 — repin to the 0.33 cohort

Schema and archive are unchanged; the minor version marks the text and API adoption.

- Pins: datom-codec eabead4 (0.33.0), protos 3c7ade3 (0.33.0), build-dependency ethos-zero
  5a5daca (19.2.0). One revision of each in the lock.
- Archive: every canonical example in `examples/canonical.datom` (4 Query, 4 Response) archives
  to the same bytes as 4.0.0 (6566d4a); `tests/archive_stability.rs` holds the 4.0.0 bytes.
- Text: every canonical example reads to the same value. The 0.33 printer breaks a line
  that holds a struct of more than two structs across lines, so `textualize` output differs from
  4.0.0 in layout only.
- Rust API: `src/generated/signal.rs` is regenerated with ethos-zero 19.2.0, so every scalar
  alias is a tuple struct. A consumer wraps a value with the constructor and reads it as `.0`;
  `RequestUnimplementedPayload` is built as
  `RequestUnimplementedPayload(UnimplementedReason::NotBuiltYet)`.
- `ReaderBudget` fields are private: build one with `ReaderBudget::try_from(n).unwrap()`.
- The contract holds no `Decimal`.
