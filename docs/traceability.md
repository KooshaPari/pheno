# Traceability Matrix

A lightweight mapping of key requirements/features to source files and tests.

| Requirement | Source | Test | Status |
|-------------|--------|------|--------|
| CLI command dispatch (cycle, plan, implement, validate, retrospective, module) | `crates/agileplus-cli/src/commands/*/mod.rs` | `crates/agileplus-cli/src/commands/*/tests.rs` | 🟡 Partial |
| REST API types & serialization | `crates/agileplus-api-types/src/` | `crates/agileplus-contract-tests/tests/` | 🟡 Partial |
| REST API handlers & routing | `crates/agileplus-api/src/` | `crates/agileplus-integration-tests/tests/` | 🟡 Partial |
| Domain models (entities, value objects) | `crates/agileplus-domain/src/` | `crates/agileplus-domain/tests/` | 🟡 Partial |
| Event bus / async messaging | `crates/agileplus-events/src/` | `crates/agileplus-events/tests/` | 🟡 Partial |
| Git repository introspection | `crates/agileplus-git/src/` | `crates/agileplus-git/tests/` | 🟡 Partial |
| GitHub PR/issue integration | `crates/agileplus-github/src/` | `crates/agileplus-github/tests/` | 🟡 Partial |
| SQLite persistence adapter | `crates/agileplus-sqlite/src/` | `crates/agileplus-sqlite/tests/` | 🟡 Partial |
| NATS messaging adapter | `crates/agileplus-nats/src/` | `crates/agileplus-nats/tests/` | 🟡 Partial |
| P2P networking layer | `crates/agileplus-p2p/src/` | `crates/agileplus-p2p/tests/` | 🟡 Partial |
| Telemetry & metrics | `crates/agileplus-telemetry/src/` | `crates/agileplus-telemetry/tests/` | 🟡 Partial |
| Caching layer | `crates/agileplus-cache/src/` | `crates/agileplus-cache/tests/` | 🟡 Partial |
| Triage / auto-review agents | `crates/agileplus-triage/src/` | `crates/agileplus-triage/tests/` | 🟡 Partial |
| Phenotype core traits | `crates/phenotype-core/src/` | `crates/phenotype-core/tests/` | 🟡 Partial |
| Python SDK (`pheno-core`) | `python/pheno-core/src/` | `python/tests/` | 🟡 Partial |
| Agent dispatch (Claude Code, Codex, PR loop) | `agileplus-agents/crates/agileplus-agent-dispatch/src/` | `agileplus-agents/crates/agileplus-agent-review/tests/` | 🟡 Partial |
| MCP server (Python) | `agileplus-mcp/src/` | `agileplus-mcp/tests/` | 🟡 Partial |
| BDD / E2E integration tests | `tests/bdd/`, `tests/integration/` | `tests/bdd/`, `tests/integration/` | 🟡 Partial |
| Contract tests (OpenAPI / gRPC) | `tests/contracts/`, `crates/agileplus-contract-tests/` | `tests/contracts/`, `crates/agileplus-contract-tests/tests/` | 🟡 Partial |
| SBOM generation | `sbom/` | `tests/integration/sbom*` | 🟡 Partial |
| CI/CD orchestration | `.github/workflows/` | `.github/workflows/ai-testing-orchestration.yml` | 🟡 Partial |

## Legend

- 🟢 Complete — source and tests aligned, passing in CI
- 🟡 Partial — source exists, tests incomplete or not yet wired in CI
- 🔴 Missing — requirement identified, no implementation or tests yet

## Notes

- This matrix is intentionally minimal; the monorepo contains 40+ crates/packages.
- For granular per-crate traceability, see individual `README.md` files under `crates/*/README.md`.
- Integration/consolidate branch: focus is on unifying test harnesses and aligning contract tests across the workspace.
