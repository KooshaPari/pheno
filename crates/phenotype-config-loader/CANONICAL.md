# CANONICAL — `phenotype-config-loader`

**Date:** 2026-06-18
**ADRs:**
- [ADR-031](https://github.com/KooshaPari/repos/blob/main/docs/adr/2026-06-17/ADR-031-configra-absorb.md) (Configra absorb)
- [ADR-022](https://github.com/KooshaPari/repos/blob/main/docs/adr/2026-06-15/ADR-022-config-consolidation-two-crate-split.md) (superseded by ADR-031, preserved for history)
- L5-104 §4 Step 3 (re-point the embedded sub-crate markers)

---

This crate is a **legacy helper** for the `pheno*` fleet. Its `load_json::<T>` /
`load_toml::<T>` helpers and the `ConfigLoadError` enum are now superseded
by the canonical config substrate.

## Canonical replacement

| Concern | Old location | New canonical location |
|---|---|---|
| Type-gated `Config` struct (URL, PORT, LOG_LEVEL, DB_PATH, FEATURE_FLAGS) + `load_from_env` / `load_from_file` / `load_from_toml_file` / `combine` / `ConfigBuilder` / `Config::merge` | `pheno-config` crate (local-only) | [`KooshaPari/Configra:crates/pheno-config/`](https://github.com/KooshaPari/Configra/tree/main/crates/pheno-config) |
| Generic JSON/TOML loaders (`load_json::<T>`, `load_toml::<T>`, `ConfigLoadError`) | `pheno/crates/phenotype-config-loader/` (this crate) | [`KooshaPari/Configra:crates/pheno-config/`](https://github.com/KooshaPari/Configra/tree/main/crates/pheno-config) — `load_from_file` + `load_from_toml_file` cover the use case |
| TS edge bindings | `KooshaPari/Conft` (archived) | [`KooshaPari/Configra:typescript/`](https://github.com/KooshaPari/Configra/tree/main/typescript) (ADR-022 split: Rust core + TS edge) |
| Hexagonal config (`settly` domain/application/adapters/infrastructure) | `KooshaPari/phenotype-config` (DEPRECATED 2026-06-17, archive 2026-07-15) | [`KooshaPari/Configra:crates/settly/`](https://github.com/KooshaPari/Configra/tree/main/crates/settly) (absorbed via [Configra PR #44](https://github.com/KooshaPari/Configra/pull/44) 2026-06-18) |

## Why this marker

Per ADR-031, `KooshaPari/Configra` (created 2026-03-25) is the canonical
Rust config substrate. The `pheno-config` standalone crate is now duplicated
in Configra as `Configra/crates/pheno-config/` (absorbed via
[Configra PR #45](https://github.com/KooshaPari/Configra/pull/45) 2026-06-18
— byte-identical 645-LoC copy). The settly hexagonal crate from the
now-deprecated `phenotype-config` is absorbed as `Configra/crates/settly/`.

## What stays here

- This `CANONICAL.md` marker
- The 64-LoC generic loader as a historical artifact (the `pheno*` fleet has
  no current consumer of this exact API after the Configra migration)
- Historical commit log

## Consumers that must update

| Consumer | Was | Now |
|---|---|---|
| `pheno*` services that load JSON/TOML into a generic type `T` | `phenotype_config_loader::load_json::<T>(path)` / `load_toml::<T>(path)` | `pheno_config::load_from_file(path)` + `pheno_config::load_from_toml_file(path)` (from `Configra/crates/pheno-config/`) — note: these are typed for the canonical `Config` struct, not generic `T` |
| `pheno*` services that want a generic `T: DeserializeOwned` loader | this crate | **No direct replacement** — define your own thin wrapper around `serde_json::from_str` / `toml::from_str`, or open a `Configra` PR to expose a `load_typed_json` / `load_typed_toml` helper |

## See also

- [ADR-031](https://github.com/KooshaPari/repos/blob/main/docs/adr/2026-06-17/ADR-031-configra-absorb.md) — Configra absorb (canonical home)
- [ADR-022](https://github.com/KooshaPari/repos/blob/main/docs/adr/2026-06-15/ADR-022-config-consolidation-two-crate-split.md) — superseded two-crate split
- [`KooshaPari/Configra`](https://github.com/KooshaPari/Configra) — canonical Rust config substrate
- [`KooshaPari/phenotype-config`](https://github.com/KooshaPari/phenotype-config) — deprecated; archive 2026-07-15
- `findings/2026-06-18-L5-110-adr-035-impl.md` (this turn) — L5-110 implementation log
