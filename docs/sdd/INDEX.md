# SDD Engine — Index

> **Status:** Reactivated 2026-06-08 per
> [ADR-0006](../adr/0006-reactivate-sdd-engine.md)

## What is SDD?

**Spec-Driven Development** is a methodology where every work item
starts with a written spec, ends with a validation gate, and produces
an evidence chain linking spec → implementation → tests → evidence.

For `pheno`, the SDD engine is the framework that makes the 21
workspace crates + 47 orphan crates coherent.

## The 5 Artifacts (per subproject)

Every subproject in `pheno` (whether a workspace crate, a new tool,
or a docs/ sub-tree) should produce these 5 artifacts:

| Artifact | Purpose | Example |
|---|---|---|
| `spec.md` | What the thing does, written as a contract | `crates/agileplus-cli/spec.md` |
| `plan.md` | How we'll build it, broken into work packages | `crates/agileplus-cli/plan.md` |
| `data-model.md` | The Rust types / DB schema (if any) | `crates/agileplus-domain/data-model.md` |
| `research.md` | Prior art, SOTA, trade-offs (one per decision) | `docs/research/sdd-engine-sota.md` |
| `validation-report.md` | Evidence the thing works as specified | `crates/agileplus-cli/validation-report.md` |

## The 19 Work Packages (WP00-WP18)

The original SDD engine defined 19 WPs covering a full
meta-spec framework. Not all 19 are required for every subproject.
The relevant ones for `pheno` in 2026 are:

| WP | Title | Status (2026-06) | Owner |
|---|---|---|---|
| WP00 | Proto-scaffold | ✅ done (in git history) | — |
| WP01 | Data model | ✅ done (in git history) | — |
| WP02 | Spec template | ✅ done (in git history) | — |
| WP03 | Plan template | ✅ done (in git history) | — |
| WP04 | Validation gate | ✅ done (in git history) | — |
| WP05 | State machine | ⏸️ deferred | — |
| WP06 | Governance | ⏸️ deferred | — |
| WP07 | Port traits | ⏸️ deferred | — |
| WP08 | SQLite adapter | ⏸️ deferred | — |
| WP09 | Git adapter | ⏸️ deferred | — |
| WP10 | Agent adapter | ⏸️ deferred | — |
| WP11 | Review adapter | ⏸️ deferred | — |
| WP12 | Telemetry adapter | ⏸️ deferred | — |
| WP13 | CLI commands | ⏸️ deferred | — |
| WP14 | gRPC surface | ⏸️ deferred | — |
| WP15 | API surface | ⏸️ deferred | — |
| WP16 | BDD tests | ⏸️ deferred | — |
| WP17 | Triage backlog | ⏸️ deferred | — |
| WP18 | Plane sync | ⏸️ deferred | — |

The deferred WPs are still defined; we just haven't executed them
in `pheno` yet. If you want to pick one up, see
[CHECKLIST.md](./CHECKLIST.md) for the validation gate.

## Validation Gate

Every subproject must pass the **validation gate** before merge.
See [CHECKLIST.md](./CHECKLIST.md) for the per-WP gate.

A simplified version of the gate (for new subprojects):

- [ ] `spec.md` exists, has 4+ sections (Purpose, Contract, Design,
      Cross-refs)
- [ ] `plan.md` exists, has at least one WP
- [ ] At least one test exists for the public API
- [ ] `cargo build -p <crate>` succeeds
- [ ] `cargo test -p <crate>` passes
- [ ] `cargo clippy -p <crate> -- -D warnings` passes
- [ ] The crate is in `[workspace] members` (no orphans)

If any of the above is missing, the PR is **not ready for review**.

## Cross-References

- [ADR-0006](../adr/0006-reactivate-sdd-engine.md) — why we
  reactivated
- [CHECKLIST.md](./CHECKLIST.md) — per-WP validation gate
- [GLOSSARY.md](./GLOSSARY.md) — SDD vocabulary
- [../../SPEC.md](../../SPEC.md) — pheno's high-level spec
- [../../PLAN.md](../../PLAN.md) — pheno's high-level plan
