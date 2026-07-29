# pheno-agents-md — SPEC

## Scope

Generate the canonical `AGENTS.md` file for a pheno-* substrate repo from a
structured spec (sections, badges, links). Implements the meta-bundle
enforcement in [ADR-039](docs/adr/2026-06-18/ADR-039-pheno-flake-refresh-template.md).

## Generated file sections

1. **Header** — repo name, tier badge, language, maturity.
2. **Purpose** — 1-paragraph scope statement.
3. **Public API** — top-level functions/classes.
4. **Conventions** — when to use, when NOT to use, 5-line quickstart.
5. **Quality bar** — 71-pillar score, test matrix, CI, license, coverage.
6. **See also** — links to relevant ADRs and health inventory.

## Spec schema (YAML)

```yaml
repo: str  # e.g. "pheno-config"
tier: 0|1|2|3|4
language: str  # e.g. "Rust"
maturity: experimental|alpha|beta|stable
purpose: str  # 1-paragraph
public_api:
  - name: str
    kind: function|class|module
    sig: str
conventions:
  when_to_use: str
  when_not_to_use: str
  quickstart: list[str]  # max 5 lines
quality:
  pillars: int  # out of 71
  tests: str
  ci: str
  license: str
  coverage: str
see_also:
  - title: str
    href: str
```

## Lint rules

- All 6 sections present.
- No dead links (HTTP 200 within 60s).
- Tier badge matches the canonical list.
- 71-pillar score is integer in [0, 71].

## CLI

```bash
pheno-agents-md generate --spec spec.yaml --out AGENTS.md
pheno-agents-md lint AGENTS.md
```

## See also

- ADR-039 — pheno-flake template
- ADR-024 — 71-pillar framework
- L6 fleet health inventory