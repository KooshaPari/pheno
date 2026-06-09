# ADR-0005: Cargo-deny and Dep-guard Enforcement

> Status: **Accepted**
> Date: 2026-06-08
> Deciders: pheno maintainers

## Context

`pheno` currently has `cargo-deny` in CI (per the existing
`.github/workflows/deny.yml`), but the configuration is permissive and
the layering rules from ADR-0004 are not enforced automatically. This
means:

- The `agileplus-cli`-imports-`agileplus-domain` layering violation
  (from ADR-0004) shipped because no automated check caught it.
- New dependencies can be added without advisory review, despite
  `cargo-deny` being available.

## Decision

Wire `cargo-deny` to enforce the **layering policy from ADR-0004** plus
the standard advisory and license checks.

Configuration in `deny.toml`:

```toml
[graph]
# Disallow the 4 known layering violations until they're fixed.
# Remove entries as violations are corrected in PRs.
forbids = [
    { name = "agileplus-cli", when = { in same crate = "agileplus-domain" } },
]

[advisories]
# Fail on any unacknowledged advisory.
version = 2
ignore = []  # no blanket ignores; case-by-case via `cargo deny` comments

[licenses]
# Phenotype ecosystem is MIT/Apache-2.0/BSD-3-Clause.
allow = ["MIT", "Apache-2.0", "BSD-3-Clause", "ISC", "Unicode-DFS-2016"]
copyleft = "warn"

[bans]
# Disallow duplicate dependencies (multiple versions of the same crate).
multiple-versions = "warn"
```

Plus a new `dep-guard` step in CI (already in place at
`.github/workflows/deny.yml`) that runs:

```
cargo deny check
cargo metadata --format-version=1 | depguard check --policy layers.toml
```

where `layers.toml` encodes the 4-layer model from ADR-0004.

## Consequences

**Positive**
- The 4 known violations become **failing CI** until fixed, not
  hidden until next refactor
- New contributors who try to add a forbidden cross-layer import
  get immediate feedback
- Dep-graph bloat (multiple versions of the same crate) is
  auto-detected

**Negative**
- Adding the depguard step adds ~30s to CI
- The 4 known violations need to be fixed in follow-up PRs before
  the deny config can be tightened
- A new dev-tooling dep (`dep-guard`) is added; needs maintenance
  if upstream stalls

## Alternatives Considered

1. **No enforcement, just code review** — rejected; code review
   already missed the violations.
2. **A custom Cargo subcommand** — rejected; `cargo-deny` covers
   most of what we need, and `dep-guard` is a small addition for
   the layering part.
3. **Replace `cargo-deny` with `cargo-audit` + `cargo-outdated`** —
   rejected; `cargo-deny` already covers advisories and licenses;
   adding a second tool is duplication.

## Cross-References

- `docs/adr/0004-inter-crate-dependency-policy.md` — the policy
- `docs/adr/ADR-015-crate-organization.md` — crate-split rationale
- `.github/workflows/deny.yml` — existing CI step (extended)
- `deny.toml` — config file (new, to be added)
