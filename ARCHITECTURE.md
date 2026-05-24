# ARCHITECTURE

## Role

`signal-upgrade` owns the peer-callable wire vocabulary for the
`upgrade` runtime. It is one leg of the `upgrade` triad beside the
runtime crate `upgrade` and the owner-only contract
`owner-signal-upgrade`.

## Boundaries

This crate owns only typed Signal records, NOTA projection derives,
frame aliases emitted by `signal_channel!`, and round-trip witnesses.
It does not own runtime orchestration, socket binding, durable storage,
migration execution, systemd unit control, or Persona handover logic.

## U1 Shape

U1 is intentionally skeletal. The channel has no domain operations yet;
it keeps the generated observability verbs and the shared
`RequestUnimplemented` reply so downstream placeholders can fail with a
typed NOTA value.

U2 populates this crate with the merged working surface from
`signal-sema-upgrade` and `signal-version-handover`: catalogue
inspection, upgrade attempts, reports, and the six handover-protocol
verbs. `Mirror` payloads remain raw bytes in their own container.

## Code Map

- `src/lib.rs` declares the scaffold channel and placeholder rejection
  records.
- `tests/round_trip.rs` proves the skeleton channel round-trips through
  NOTA and Signal frames.
- `examples/canonical.nota` records the current placeholder text shape.

## Invariants

- Contract operation roots are contract-local verbs in verb form.
- The contract crate carries no daemon, actor, database, or Tokio
  runtime code.
- The ordinary and owner contracts remain separate repositories.
- This crate depends on `version-projection`; U2 consumes its
  `ComponentName`, `ContractVersion`, and `RecordKind` vocabulary.
