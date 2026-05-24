# skills - signal-upgrade

Read this before editing the ordinary upgrade contract.

## Required Context

- `~/primary/skills/contract-repo.md`
- `~/primary/skills/component-triad.md`
- `~/primary/skills/architectural-truth-tests.md`
- `~/primary/skills/nix-discipline.md`
- this repo's `ARCHITECTURE.md`

## Boundary

This crate owns only the ordinary `upgrade` Signal vocabulary. It has no
runtime, actors, sockets, storage, migration modules, Persona handover
driver, or systemd integration.

## Invariants

- U1 stays scaffold-only. Do not move `sema-upgrade`,
  `signal-sema-upgrade`, `signal-version-handover`, or Persona code
  into this crate in U1.
- U2 is the first population step for the merged working contract.
- `Mirror` payloads stay raw bytes when U2 lands.
- `RequestUnimplemented` stays available so partial implementations
  can return typed skeleton replies.
- Round-trip tests cover both NOTA and Signal-frame encoding.
