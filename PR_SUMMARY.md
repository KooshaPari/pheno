# Merge Prep Summary — `integration/consolidate`

## What this consolidates
- **Documentation & Governance** — traceability matrix for top features, SECURITY.md stub, CONTRIBUTING.md refresh, FUNDING.yml, deterministic build-status docs
- **CI & Workflow Hygiene** — pin GitHub Actions to immutable SHAs, migrate runners to `ubuntu-24.04`, add workflow permissions, integrate TruffleHog secret scanning
- **Repository Hygiene** — hardened `.gitignore` / `.dockerignore`, normalized root `.editorconfig`, updated LICENSE author, removed stale RUSTSEC ignores and broken-reference manifests
- **Linting** — applied `golangci-lint` fixes across `pheno` and `pheno-cli` source/cmd packages
- **Build & Dependency Fixes** — switched `phenotype-http-client-core` to `rustls-tls` (drops `openssl-sys`), restored workspace members list, deduplicated compose files via symlinks to shelf-root canonicals
- **Security Hardening** — added vulnerability reporting guidance, tightened `deny.toml` wildcard policy

## Tests added
- `test/onefn4` and `test/cover-5` branches merged
- Focused unit tests for `Duration::format_compact` (`phenotype-time`)
- Focused unit tests for `Timestamp::parse` (`phenotype-time`)

## Traceability
- Traceability matrix skeleton created and populated for top 5 features
- Matrix maps requirements → implementation → tests for consolidated features

## Build status
- Deterministic build status documented for this branch
- `Cargo.lock` conflicts resolved and committed
- `deny.toml` updated with current policy and audit state

## Merge risk
**Low** — all changes are non-breaking (docs, chore, lint, test, CI). No API surface changes. Workspace and dependency fixes are restorative rather than invasive. Conflicts with `main` have already been resolved during branch consolidation.
