# Migration: settly → phenotype-config

**Date:** 2026-06-17  
**Disposition row:** HexaKit DISPOSITION #45 — `crates/settly`  
**Canonical repo:** https://github.com/KooshaPari/phenotype-config  
**RFC:** [RFC 002 — Settly config role](https://github.com/KooshaPari/phenotype-registry/blob/main/docs/rfc/002-settly-config-role.md)

## What changed

- Removed `crates/settly` from the HexaKit workspace members (Wave 2 excision).
- Canonical `config` role ownership is **`phenotype-config`** (`crates/settly`).
- **Source tree removed** from HexaKit; only this redirect stub remains until archive-delete boundary.

## For consumers

1. Depend on `settly` from **phenotype-config**, not HexaKit `crates/settly`.
2. Git dependency (fleet default):

```toml
settly = { git = "https://github.com/KooshaPari/phenotype-config", branch = "main" }
```

3. Pyron repointed in lockstep — see [Pyron migration note](https://github.com/KooshaPari/Pyron/blob/main/docs/migrations/settly-repoint-2026-06-17.md).

## For HexaKit maintainers

- Do not add new config-domain code under `crates/settly`.
- Remove this stub directory once zero external path deps remain.
