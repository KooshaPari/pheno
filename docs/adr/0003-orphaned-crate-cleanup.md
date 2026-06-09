# ADR-0003: Orphaned Crate Cleanup

> Status: **Accepted**
> Date: 2026-06-08
> Deciders: pheno maintainers

## Context

`pheno` is a Cargo workspace with **68 crates on disk** but only **21 listed in
`[workspace] members`** in the root `Cargo.toml`. The 47 crates that are NOT
in the workspace are silently ignored by `cargo build`, `cargo test`, and
`cargo metadata` — yet they consume disk space, show up in `git status` for
untracked-but-tracked-but-not-built confusion, and risk being referenced by
_stale_ documentation or scripts.

The 47 orphans break down by size and reference state:

| Size bucket | Count | Notes |
|---|---|---|
| > 1000 LOC | 12 | Likely real work that was removed from workspace for a reason |
| 100-1000 LOC | 18 | Probably experiments, scratch crates, or in-progress work |
| < 100 LOC | 17 | Stubs, scaffolds, or near-empty crates |

| Reference state | Count | Notes |
|---|---|---|
| Referenced by another crate's `Cargo.toml` | 0 | No _Cargo_ dependencies from workspace crates |
| Referenced in any `.md` file | < 5 | Some docs still mention old names |

## Decision

**Adopt a 3-step decision process** for the 47 orphans, executed in order:

1. **Promote** (target: 0-3 crates) — If an orphan is referenced by recent docs
   or scripts, add it to `[workspace] members` and run `cargo test` to verify
   it still builds. Document the promotion in a CHANGELOG entry.

2. **Move** (target: 0-2 crates) — If an orphan is a real artifact that belongs
   in a different repo, create a sibling repo and `git mv` the crate there. Keep
   a tombstone (a single `.gitkeep` + a comment in PLAN.md) so future contributors
   don't re-invent the wheel.

3. **Delete** (target: 42-47 crates) — If an orphan has no references, no
   recent activity, and is not on a docs-as-spec hot path, `git rm` the crate.
   This is reversible (every removed crate is preserved in git history).

The first batch (47 orphans) will be processed in 1 commit per decision
type, with each commit naming the affected crates. A `pheno-orphan-tracking.md`
file at the workspace root will be created to record the per-crate rationale.

## Consequences

**Positive**
- `cargo build --workspace` covers the actual code surface
- Disk space reclaimed (47 orphan crates × ~5 MB average = ~235 MB)
- Cleaner separation between "shipping" and "experimental" code
- Honest representation: if something isn't in the workspace, it isn't a
  pheno deliverable

**Negative**
- Risk of deleting a crate that someone was about to wire up
- Documentation may reference deleted crate names (will need follow-up)
- Some build scripts or CI may have hardcoded paths to orphan crates

**Mitigations**
- Each deletion is a single commit, individually revertable
- The orphan-tracking document records per-crate rationale
- A pre-deletion `git status` check identifies references before removal
- A 7-day review window (PR-style) is recommended before the delete batch
  lands on `main`

## Alternatives Considered

1. **Leave all 47 orphans in place** — rejected; this is the status quo that
   brought us here. Disk bloat + cognitive load + "does cargo build this?" ambiguity.
2. **Add all 47 to the workspace** — rejected; many of them are stubs and adding
   them forces a triage of what compiles vs what doesn't, with no upside.
3. **Hard-delete without tracking** — rejected; the orphan-tracking document
   preserves institutional knowledge ("we considered X and removed it for reason Y").

## Cross-References

- `SPEC.md` § "Workspace Layout" — current 21-crate layout
- `docs/adr/ADR-015-crate-organization.md` — pre-existing crate-split rationale
- `PLAN.md` § "Decomposition Plan" — execution phases
- `pheno-orphan-tracking.md` (to be created) — per-crate decision log
