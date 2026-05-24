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

## Working Shape

The channel is the merged ordinary upgrade surface:

- `Inspect`, `AttemptUpgrade`, and `Report` expose compiled migration
  support and historical outcomes.
- `AskHandoverMarker`, `ReadyToHandover`, `HandoverCompleted`,
  `Mirror`, `Divergence`, and `RecoverFromFailure` carry the
  adjacent-version handover protocol.
- `RequestUnimplemented` remains as the typed placeholder reply for
  operations whose runtime integration has not landed yet.

`Mirror` and `Divergence` payloads carry raw bytes in their own typed
containers. The projection policy for those bytes lives in
`version-projection`; the execution and persistence policy lives in the
`upgrade` runtime.

## Code Map

- `src/lib.rs` declares the merged catalogue and handover channel.
- `tests/round_trip.rs` proves the merged channel round-trips through
  NOTA and Signal frames.
- `examples/canonical.nota` records stable catalogue text examples.

## Invariants

- Contract operation roots are contract-local verbs in verb form.
- The contract crate carries no daemon, actor, database, or Tokio
  runtime code.
- The ordinary and owner contracts remain separate repositories.
- This crate depends on `version-projection`; handover records use its
  `ComponentName`, `ContractVersion`, and `RecordKind` vocabulary.
