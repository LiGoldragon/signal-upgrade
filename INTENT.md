# INTENT — signal-upgrade

*The ordinary peer-callable wire contract for the `upgrade` runtime.
Defines the typed channel for upgrade-catalogue inspection, upgrade
attempts, and the adjacent-version handover protocol. One leg of the
`upgrade` triad beside the runtime crate `upgrade` and the meta policy
contract `meta-signal-upgrade`. Companion to `ARCHITECTURE.md` and
`Cargo.toml`. Maintenance: `primary/skills/repo-intent.md`.*

## Repo-scope only

This file carries only the intent that is FOR this `signal-upgrade`
contract. Workspace-shape intent stays in the primary workspace
`primary/INTENT.md`. Component daemon intent stays in
`upgrade/INTENT.md`. Meta policy and selector authority intent
stays in `meta-signal-upgrade/INTENT.md`.

## Why this repo exists

`signal-upgrade` is the **ordinary peer-callable wire contract** for the
`upgrade` daemon. It is the merged ordinary upgrade surface (per the
`signal-version-handover` + `signal-sema-upgrade` merge): catalogue
inspection and outcome reporting (`Inspect`, `AttemptUpgrade`, `Report`)
together with the adjacent-version handover protocol (`AskHandoverMarker`,
`ReadyToHandover`, `HandoverCompleted`, `Mirror`, `Divergence`,
`RecoverFromFailure`). Meta catalogue policy and emergency selector
control stay in `meta-signal-upgrade`; runtime orchestration, migration
execution, sockets, and durable storage live in `upgrade`.

## The channel shape

The upgrade channel carries:

- **Catalogue / attempt:** `Inspect` and `Report` expose compiled
  migration support and historical outcomes; `AttemptUpgrade` requests a
  migration attempt.
- **Handover protocol:** `AskHandoverMarker`, `ReadyToHandover`,
  `HandoverCompleted`, `Mirror`, `Divergence`, `RecoverFromFailure`
  carry the daemon-to-daemon protocol two versions of one component run
  to move the public surface without losing writes.
- **Replies:** typed per operation, with `RequestUnimplemented` as the
  skeleton-honest placeholder for operations whose runtime integration
  has not landed.

`Mirror` and `Divergence` payloads carry raw bytes in their own typed
containers. Projection policy for those bytes lives in
`version-projection`; execution and persistence policy lives in the
`upgrade` runtime.

## Channels are closed, boundaries are named

- Wire enums are closed. No `Unknown` escape hatch.
- Request payloads do not mint policy revisions, timestamps, or
  authority sequences; the daemon mints those.
- No stringly-typed dispatch. Status and reason fields are typed closed
  enums.

## Wire vocabulary discipline

Per `primary/skills/contract-repo.md` §"Public contracts use
contract-local operation verbs":

- Operation roots are domain verbs in verb form (`Inspect`,
  `AttemptUpgrade`, `Report`, the handover verbs). The six Sema
  classification words must not appear as request roots on this wire.
- Reply success variants name the concrete outcome the daemon produced.
- Payload record names are the domain nouns the operation carries; raw
  handover bytes ride in typed containers, never generic wrappers.

## Schema-derived stack

This contract is migrating to the schema-derived stack. `schema/lib.schema`
declares the schema-next source for the ordinary upgrade signal surface;
`build.rs` deserializes it into `SchemaSource`, validates the
schema-in-Rust value through text and rkyv round-trips, and fails the
build when the generated `src/schema/lib.rs` is stale. The generated
module is emitted with the `schema-rust-next` `WireContract` target, so it
carries wire types and codecs ONLY — zero engine traits. Daemon-internal
Signal/Nexus/SEMA plane schemas live inside the `upgrade` runtime crate,
never in this external contract repository.

Text projection is explicit. The default contract dependency graph is
binary-only and must not pull `nota-next`, `nota-codec`, or
`signal-core`; `nota-text` enables the generated and hand-written NOTA
derives/impls for CLI and witness builds. Runtime daemons consume this
contract with default features disabled/empty and speak rkyv frames.

## Constraints

- This crate carries only typed wire vocabulary, optional NOTA
  projection derives, frame aliases, and round-trip witnesses.
- No runtime code: no daemon, no actors, no tokio, no socket binding, no
  database, no migration execution, no systemd unit control, no handover
  logic.
- The ordinary and meta contracts remain separate repositories.
- This crate depends on `version-projection`; handover records use its
  `ComponentName`, `ContractVersion`, and `RecordKind` vocabulary.
- Every operation and reply round-trips through NOTA and Signal frames;
  the generated module is guarded against Nexus/SEMA runtime terms,
  trace/mail helpers, and generic plane envelopes.
- Wire dependency pins use named branches or tags, not raw revision
  hashes.

## Non-ownership

This crate does not own:

- `upgrade` daemon runtime, actors, or component lifecycle;
- `upgrade.sema` or catalogue/selector/migration state tables;
- socket binding, transport, the private upgrade socket, or version
  handshake policy;
- migration execution, handover state machine, or selector logic;
- meta catalogue policy and selector authority
  (`meta-signal-upgrade`);
- raw-byte projection policy (that is `version-projection`).

## See also

- `ARCHITECTURE.md` — detailed working shape, the merged surface, the
  schema-next migration, and the byte-container discipline.
- `../upgrade/INTENT.md` — daemon-side intent (schema-driven planes,
  migration orchestration, handover driver).
- `../meta-signal-upgrade/INTENT.md` — meta catalogue policy and
  selector authority contract.
- `../version-projection/ARCHITECTURE.md` — projection library for
  handover record bytes.
- `primary/skills/contract-repo.md` — contract repo discipline and
  naming rules.
- `primary/skills/component-triad.md` — repo triad structure and wire
  layers.
