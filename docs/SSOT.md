# SSOT — pheno

## State
- Default branch: main
- Last verified: 2026-06-08
- CI status: green
- Open PRs: 0
- Open branches: 1 (main)
- Stashes: 0

## Architecture
- Hexagonal: yes (per crate)
- Ports: various (see individual crate docs)
- Adapters: various (see individual crate docs)
- Domain: Phenotype core platform

## Merges
- phenoData -> crates/pheno-data-from-phenoData/ (2026-06-08)

## Next Steps
1. [ ] Integrate phenoData crates into pheno workspace
2. [ ] Merge Cargo.toml workspace manifests
3. [ ] Deduplicate shared dependencies (tokio, serde, etc.)
4. [ ] Add data layer tests to pheno CI
