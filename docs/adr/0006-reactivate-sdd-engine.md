# ADR-0006: Reactivate Archived SDD Engine

> Status: **Accepted**
> Date: 2026-06-08
> Deciders: pheno maintainers

## Context

`pheno` has a **`spec-driven-development-engine`** subdirectory in its
git history, archived at some point in 2024-2025. The SDD engine
prescribes:

- A meta-spec framework with a `data-model.md`, `plan.md`, `spec.md`,
  `research.md`, `validation-report.md` per subproject
- 19 work packages (WP00-WP18) covering the full implementation
  surface
- A validation gate at the end of each WP

In its absence, `pheno` has accumulated:
- Inconsistent `.rs` file organization across crates
- 17 crates with no test files (per the audit)
- 47 orphan crates on disk not in `[workspace] members`
- Test names that don't follow a discoverable convention

## Decision

**Reactivate the SDD engine** as the meta-spec for ongoing work in
`pheno`. Apply it in two phases:

### Phase 1: Foundation (this session)

- Re-add `docs/sdd/` with the engine's index, principles, and a
  glossary
- Add a 1-page summary of the WP00-WP18 work-package structure
  to `docs/sdd/INDEX.md`
- Add `docs/sdd/GLOSSARY.md` defining the SDD vocabulary
  ("spec-driven", "validation gate", "evidence chain", etc.)
- Add `docs/sdd/CHECKLIST.md` for the per-WP validation gate

### Phase 2: Adoption (next session)

- For each of the 21 workspace crates, create a
  `docs/crates/<crate>/spec.md` using the SDD engine's template
- For each, define the validation gate
- For the 17 zero-test crates, the first WP must be "add at least
  one test per public function"

### Out of scope for this session

- Full WP00-WP18 re-execution. That's 19 WPs of work, each
  several hours. Deferred to a separate "execution" session.

## Consequences

**Positive**
- New contributors have a discoverable framework for understanding
  what each crate is for
- Inconsistencies across crates can be reduced by applying the
  same template everywhere
- The validation gate prevents "looks good" PRs that don't actually
  add tests

**Negative**
- Initial overhead: each crate needs a `spec.md` (one-time)
- The WP00-WP18 re-execution is significant work; if not
  prioritized, the engine reactivates in name only

## When to Revisit

This ADR should be re-evaluated in Q3 2026:
- Has the per-crate `spec.md` been written for all 21 crates?
- Are validation gates actually catching missing tests in PRs?
- If not, the engine is decoration; consider deprecating it.

## Cross-References

- `docs/sdd/INDEX.md` (new) — engine index
- `docs/sdd/CHECKLIST.md` (new) — per-WP checklist
- `SPEC.md` (new) — pheno's high-level spec, references the engine
- `docs/adr/0003-orphaned-crate-cleanup.md` — one WP of the engine
- `docs/adr/0004-inter-crate-dependency-policy.md` — one WP of the engine
