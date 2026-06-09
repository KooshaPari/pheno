# ADR-0004: Inter-Crate Dependency Policy

> Status: **Accepted**
> Date: 2026-06-08
> Deciders: pheno maintainers

## Context

`pheno` is a 21-crate Cargo workspace. As the crate count grows, it becomes
easy to introduce **cycle dependencies**, **layering violations** (e.g. a
library crate depending on a binary crate), and **excessive coupling** that
makes refactoring painful. A previous audit found:

- 3 crates that depend on each other transitively via feature flags
- 1 binary crate (`agileplus-cli`) imported by a library crate
  (`agileplus-domain`) — a layering violation
- 2 crates that re-implement the same trait (no shared abstraction)

These all happened because there was no documented rule for which crates
may import which.

## Decision

Adopt a **layered dependency policy** for the 21 workspace crates, organized
into 4 layers:

| Layer | Crates | May import |
|---|---|---|
| L0: Foundation | (none yet) | std + external crates only |
| L1: Core domain | `agileplus-domain` | L0 |
| L2: Adapters | `agileplus-sqlite`, `agileplus-traits` | L0, L1 |
| L3: Application | `agileplus-api`, `agileplus-cli`, `agileplus-subcmds` | L0, L1, L2 |

Rules:
- A crate at layer N may ONLY import crates at layers < N.
- Binary crates (anything with `[[bin]]`) are **leaves**: no other crate
  may import them. The `agileplus-cli` importing `agileplus-domain`
  violation is corrected by extracting a `agileplus-domain-ops` library
  that `agileplus-cli` imports _instead_ of `agileplus-domain`.
- Trait duplication is consolidated into `agileplus-traits`. Any new
  trait goes there, not into the concrete impl.

The current 21 crates will be re-mapped onto this 4-layer model in the
"Layer Assignment" section of SPEC.md. Any crate that doesn't fit is
either promoted to a new layer (with justification in a new ADR) or
deleted under the orphan-cleanup policy (ADR-0003).

## Consequences

**Positive**
- Cycles are structurally impossible (acyclic by construction)
- Layering violations are caught at code-review time by the
  `cargo-deny` + a custom workspace-level check (see ADR-0005)
- New contributors can place new crates correctly by following
  the layer model
- Refactoring is bounded: changing L1 cannot ripple to L3 without
  explicit re-wiring

**Negative**
- The current code violates the policy in 4-5 places; fixing them
  is a multi-PR effort
- The policy is a "soft" rule until `cargo-deny` is configured to
  enforce it (separate work item)
- Some valid cross-layer imports may be artificially broken (e.g. an
  L3 crate legitimately needing to test an L1 invariant) — these need
  a workaround (test-only re-exports)

## Alternatives Considered

1. **No policy** — rejected; status quo brought us the violations.
2. **Cargo's built-in "workspaces don't enforce layering"** — true,
   but we can layer the policy on top via `cargo-deny` and code review.
3. **Split into 4 separate workspaces** — rejected; too much
   tooling friction (4 sets of `Cargo.lock`, 4 CI matrices, etc.)
   for the benefit. The 4-layer model captures the same intent with
   less overhead.

## Cross-References

- `SPEC.md` § "Workspace Layout" — current 21 crates
- `docs/adr/ADR-015-crate-organization.md` — pre-existing crate-split rationale
- `docs/adr/0003-orphaned-crate-cleanup.md` — companion ADR
- `PLAN.md` § "Decomposition Plan" — phasing
