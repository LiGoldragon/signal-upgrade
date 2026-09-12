# ARCHITECTURE

## Role

`signal-upgrade` owns the peer-callable wire vocabulary for the
`upgrade` runtime. It is one leg of the `upgrade` triad beside the
runtime crate `upgrade` and the meta policy contract
`meta-signal-upgrade`.

## Boundaries

This crate owns only typed Signal records, optional Datom text
projection derives, generated `signal-frame` aliases/codecs, and round-trip
witnesses. It does not own runtime orchestration, socket binding,
durable storage, migration execution, systemd unit control, or Persona
handover logic. It also does not own meta catalogue policy or selector
authority (those stay in `meta-signal-upgrade`), the private upgrade
socket or version-handshake policy, or raw-byte projection policy below
this wire contract. Daemon-internal Signal/Nexus/SEMA plane schemas live
inside the `upgrade` runtime crate, not in this external contract
repository.

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
containers. Projection policy for those bytes belongs below the
contract, in runtime/library code; execution and persistence policy
lives in the `upgrade` runtime.

## Code Map

- `schema/lib.schema` declares the first real schema-next source for
  the ordinary upgrade signal surface and its generated wire-only
  Input/Output roots.
- `src/schema/lib.rs` is the checked-in generated Rust interface;
  `build.rs` deserializes `schema/lib.schema` into `SchemaSource`,
  validates the schema-in-Rust value through text and rkyv round-trips,
  and fails the build when the generated Rust is stale.
- `src/lib.rs` re-exports the generated schema module as the crate's
  public contract API.
- `tests/contract.rs` proves the merged channel round-trips through
  Signal frames in default mode and through Datom text under the
  `datom` feature.
- `examples/canonical.datom` records stable catalogue text examples.

## Invariants

- Contract operation roots are contract-local verbs in verb form
  (`Inspect`, `AttemptUpgrade`, `Report`, the handover verbs); the six
  Sema classification words must not appear as request roots on this
  wire.
- Wire enums are closed. No `Unknown` escape hatch.
- No stringly-typed dispatch. Status and reason fields are typed closed
  enums.
- Request payloads do not mint policy revisions, timestamps, or
  authority sequences; the daemon mints those.
- The contract crate carries no daemon, actor, database, or Tokio
  runtime code.
- The generated module carries wire types/codecs only.
- Datom text parsing/rendering is feature-gated under `datom`; the
  default contract graph is binary-only for daemon consumers.
- The ordinary and meta contracts remain separate repositories.
- Handover records use contract-local `ComponentName`,
  `ContractVersion`, and `RecordKind` wire nouns. Projection policy is
  not part of this public Signal contract.
- Wire dependency pins use named branches or tags, not raw revision
  hashes.

## Schema-derived contract

**Status:** migrated. The crate's public API is emitted from
`schema/lib.schema`; there is no parallel hand-written channel surface.

`schema-rust-next` emits the wire types, short-header projection,
request/reply frame aliases, and binary codecs. It does not emit daemon
runtime planes here.

**Per-component concerns:**
- The merged ordinary surface from `signal-version-handover` and
  `signal-sema-upgrade` is declared in one schema file.
- `Mirror` and `Divergence` payloads carry raw bytes in typed
  containers; the schema declares those byte-carrying record shapes,
  but projection policy stays below the contract and execution policy
  stays in the `upgrade` runtime.

**References:**
- `reports/designer/326-v13-spirit-complete-schema-vision.md` - uniform header form + schema-language design
- `reports/designer/324-migration-mvp-spirit-handover-re-specification.md` - migration MVP + handover state
- `reports/designer/322-spirit-mvp-positional-schema-worked-example.md` - Spirit MVP worked example
- `reports/operator/174-schema-import-header-design-critique-2026-05-24.md` - header/body/feature separation + lowering rules
