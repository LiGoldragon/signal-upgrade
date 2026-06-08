# ARCHITECTURE

## Role

`signal-upgrade` owns the peer-callable wire vocabulary for the
`upgrade` runtime. It is one leg of the `upgrade` triad beside the
runtime crate `upgrade` and the meta policy contract
`meta-signal-upgrade`.

## Boundaries

This crate owns only typed Signal records, optional NOTA projection
derives, frame aliases emitted by `signal_channel!`, and round-trip
witnesses. It does not own runtime orchestration, socket binding,
durable storage, migration execution, systemd unit control, or Persona
handover logic. Daemon-internal Signal/Nexus/SEMA plane schemas live
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
containers. The projection policy for those bytes lives in
`version-projection`; the execution and persistence policy lives in the
`upgrade` runtime.

## Code Map

- `schema/lib.schema` declares the first real schema-next source for
  the ordinary upgrade signal surface and its generated wire-only
  Input/Output roots.
- `src/schema/lib.rs` is the checked-in generated Rust interface;
  `build.rs` deserializes `schema/lib.schema` into `SchemaSource`,
  validates the schema-in-Rust value through text and rkyv round-trips,
  and fails the build when the generated Rust is stale.
- `src/lib.rs` declares the merged catalogue and handover channel.
- `tests/round_trip.rs` proves the merged channel round-trips through
  Signal frames in default mode and through NOTA under `nota-text`.
- `tests/dependency_boundary.rs` pins the feature boundary: default
  builds do not pull `nota-next`, `nota-codec`, or `signal-core`;
  `nota-text` is the explicit text-codec opt-in.
- `tests/generated_schema.rs` exercises generated Input/Output
  short-header/frame round-trips and guards against generated
  Nexus/SEMA runtime terms, trace/mail helpers, and generic plane
  envelopes in this contract.
- `examples/canonical.nota` records stable catalogue text examples.

## Invariants

- Contract operation roots are contract-local verbs in verb form.
- The contract crate carries no daemon, actor, database, or Tokio
  runtime code.
- The generated schema module is emitted with `schema-rust-next`
  `WireContract` target, so it carries wire types/codecs only.
- NOTA parsing/rendering is feature-gated under `nota-text`; the
  default contract graph is binary-only for daemon consumers.
- The ordinary and meta contracts remain separate repositories.
- This crate depends on `version-projection`; handover records use its
  `ComponentName`, `ContractVersion`, and `RecordKind` vocabulary.

## Pending schema-engine upgrade

**Status:** migration started. The crate now carries checked-in
schema-next artifacts beside the hand-written `signal_channel!`
surface. The generated module is a witness surface until the runtime
cutover replaces the hand-written contract path.

**Target:** this crate's hand-written `signal_channel!` invocation + typed records (catalogue inspection, attempt-upgrade verb, handover-protocol verbs, mirror/divergence/recovery records) converts fully to the checked-in `schema/lib.schema` source. `schema-rust-next` emits the wire types, ShortHeader projection, dispatcher, VersionProjection, and storage descriptors for any consumer-side persistence.

**Sequence:** Spirit pilots `primary-ezqx.1` first; this contract's schema cutover then absorbs the merged surface (`AttemptUpgrade` + handover-protocol verbs) into one schema file as part of the upgrade-triad-as-schema-host work named in the `upgrade` runtime's ARCH. Because the upgrade triad is the natural home for the schema-daemon registry (per /326-v13 §4), this contract's cutover may land tightly coupled with the runtime's, not as a separate operator pass.

**Per-component concerns:**
- Merged from `signal-version-handover` + `signal-sema-upgrade` per /318; the schema cutover absorbs `AttemptUpgrade` + the handover-protocol verbs (`AskHandoverMarker`, `ReadyToHandover`, `HandoverCompleted`, `Mirror`, `Divergence`, `RecoverFromFailure`) into one schema file.
- `Mirror` and `Divergence` payloads carry raw bytes in typed containers; the schema declares those byte-carrying record shapes, but projection policy stays in `version-projection` and execution policy stays in the `upgrade` runtime.
- Depends on `version-projection` for `ComponentName`, `ContractVersion`, `RecordKind`; the schema imports that vocabulary from `version-projection`'s own macro-pattern integration (see Slice D's substrate-library marking).

**References:**
- `reports/designer/326-v13-spirit-complete-schema-vision.md` — uniform header form + schema-language design
- `reports/designer/324-migration-mvp-spirit-handover-re-specification.md` — migration MVP + handover state
- `reports/designer/322-spirit-mvp-positional-schema-worked-example.md` — Spirit MVP worked example
- `reports/operator/174-schema-import-header-design-critique-2026-05-24.md` — header/body/feature separation + lowering rules
