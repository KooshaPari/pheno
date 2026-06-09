# pheno — Specification

> Status: **Living document** — updated as the workspace evolves.
> Last updated: 2026-06-08

## Purpose

`pheno` is a multi-domain Rust monorepo that consolidates the `agileplus*` family
of crates (and several standalone Phenotype platform tools) under a single Cargo
workspace. The `agileplus` product is a CLI/API for a sprint-style task management
workflow; the platform tools are supporting libraries that other Phenotype services
consume.

This is a **library + binary workspace**. The `agileplus-cli` crate is the primary
binary; everything else is a library consumed either by `agileplus-cli` or by
external services.

## Workspace Layout

```
pheno/
├── crates/
│   ├── agileplus-cli/          # primary binary (8.9K LOC)
│   ├── agileplus-api/          # HTTP/RPC server (6.7K LOC)
│   ├── agileplus-sqlite/       # persistence layer (5.7K LOC)
│   ├── agileplus-domain/       # core types + business rules (4.4K LOC)
│   ├── agileplus-subcmds/      # CLI subcommand dispatch (4.4K LOC)
│   └── ... 16 more agileplus-* crates ...
│
│   # 47 ORPHANED crates on disk (not in [workspace] members) — see ADR-0003
│
├── docs/
│   ├── adr/                    # Architecture Decision Records
│   ├── operations/             # operational runbooks
│   └── worklogs/               # dated worklog entries
├── tests/                      # cross-crate integration tests
├── target/                     # cargo build cache (gitignored)
├── Cargo.toml                  # workspace root
├── Cargo.lock                  # resolved dependency graph
└── SPEC.md                     # this file
```

## Design Principles

1. **Workspace member first, standalone second** — every crate lives inside the
   `pheno` workspace unless it has a specific reason to be a standalone crate
   (e.g. external publication to crates.io). Standalone publication is a deliberate
   action, not a default.
2. **Hexagonal domain/persistence split** — `agileplus-domain` contains the business
   logic; `agileplus-sqlite` contains the persistence adapter. The domain has no
   `sqlx` or `diesel` dependency.
3. **CLI / API / domain triangle** — `agileplus-cli` and `agileplus-api` both
   consume `agileplus-domain`; they share no code with each other.
4. **No orphan crates** — every crate in `crates/` is either in `[workspace]`
   members OR is documented in `docs/adr/` with a reason. (See ADR-0003.)
5. **Bounded test scope per crate** — each crate has a `tests/` dir for integration
   tests; unit tests live next to the code they test.
6. **Bounded CI** — `cargo test --workspace` must complete in < 10 min on
   CI; a crate that pushes us over the limit must be split or feature-gated.

## Crate Contracts (the 21 workspace members)

### `agileplus-cli`

- **Role**: Primary CLI binary. Parses argv, dispatches to subcommands.
- **Public surface**: `main()` only (binary crate).
- **Errors**: error-chain types mapped to exit codes.
- **Tests**: integration tests for each subcommand via `assert_cmd` or `clap`-test.

### `agileplus-api`

- **Role**: HTTP/RPC server.
- **Public surface**: `serve()`, route handlers.
- **Errors**: HTTP status code mapping.
- **Tests**: request/response contract tests with `axum::Router`.

### `agileplus-sqlite`

- **Role**: SQLite-backed persistence implementing the
  `agileplus-domain::Repository` trait.
- **Public surface**: `SqliteRepository::open`, `SqliteRepository::migrate`.
- **Errors**: typed `PersistenceError` enum.
- **Tests**: in-memory SQLite + migration tests + round-trip tests.

### `agileplus-domain`

- **Role**: Core types (Task, Project, Sprint) and business rules
  (state machines, validation).
- **Public surface**: domain types + `Repository` trait + pure functions.
- **Errors**: `DomainError` enum.
- **Tests**: pure-function unit tests, no I/O.
- **No `sqlx` / `diesel` / `tokio::fs` dependency** — this crate is pure logic.

### `agileplus-subcmds`

- **Role**: Subcommand implementation (one module per subcommand).
- **Public surface**: `register_subcommands(&mut App)`.
- **Errors**: per-subcommand error types.
- **Tests**: per-subcommand argument parsing tests.

*(See `docs/adr/` for individual decisions on each of the 21 workspace crates.)*

## Cross-Cutting Concerns

- **Logging**: `tracing` crate; structured fields only.
- **Configuration**: `config` crate or env vars; no secret in source.
- **Telemetry**: OpenTelemetry-compatible; behind feature flag.
- **Migrations**: `agileplus-sqlite` owns all schema migrations; no
  `migrations/` dir at the workspace root.
- **Feature flags**: each crate exposes `default = []`; the workspace root
  can compose feature sets for binary builds.

## Test & Coverage Governance

- **Coverage floor**: 70% line coverage per crate; enforced by `tarpaulin.toml`
  and surfaced via Codecov.
- **Coverage report**: posted as PR comment and archived as
  `tarpaulin-report.html` artifact.
- **BDD**: at least one `.feature` file per crate covering the happy-path
  user journey (see `tests/features/`).
- **CI matrix**: tests must pass on stable; clippy must pass with `-D warnings`.
- **No-crate-without-tests**: every workspace member must have at least
  one `#[test]` or `tests/*.rs`. (See ADR-0004 for the 17-crute test gap.)

## Decomposition Roadmap

Phase 1 (this PR): governance triangle + orphan decision (ADR-0003).
Phase 2: reactivate the archived `spec-driven-development-engine` (see
`docs/worklogs/2026-04-22-sdd-engine-sunset.md`) to drive the refactor.
Phase 3: split `agileplus-*` into a separate repo OR keep them grouped
under a `crates/agileplus/` subdir.
Phase 4: write BDD .feature files for each of the 21 workspace crates.

## Open Questions

- Should the `agileplus-*` crates move to a standalone `agileplus` repo?
  (Tracked in `docs/adr/0006-agileplus-stand-alone-repo.md` — pending.)
- Should we re-activate the archived SDD engine to drive future refactors?
  (Tracked in `docs/adr/0007-sdd-engine-reactivation.md` — pending.)
- Should we add a `pheno-cli` (general-purpose) alongside `agileplus-cli`?
  (Tracked in `docs/adr/0008-rename-or-add-pheno-cli.md` — pending.)

## Cross-References

- `FUNCTIONAL_REQUIREMENTS.md` — high-level FRs and acceptance criteria
- `AGENTS.md` — agent operating instructions
- `docs/adr/0001-record-architecture-decisions.md` — ADR template
- `docs/adr/0002-workspace-membership-policy.md` — 21-member rule
- `docs/adr/0003-orphan-crate-triage.md` — what to do with the 47 orphans
- `docs/adr/0004-no-crate-without-tests.md` — 17 zero-test crates
- `docs/adr/0005-cargo-workspace-decomposition.md` — `crates/agileplus/`
- `docs/adr/0006-agileplus-stand-alone-repo.md` — pending
- `docs/adr/0007-sdd-engine-reactivation.md` — pending
- `docs/adr/0008-rename-or-add-pheno-cli.md` — pending
