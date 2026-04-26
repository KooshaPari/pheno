# AGENTS.md — pheno

Phenotype Shared Crates (Rust) — Agent Rules

## Quick Links

- **Local CLAUDE.md:** See `CLAUDE.md` in this repository for project-specific guidance
- **Phenotype org governance:** `/Users/kooshapari/CodeProjects/Phenotype/repos/CLAUDE.md`
- **Global agent guidance:** `~/.claude/AGENTS.md`
- **AgilePlus work tracking:** `cd /Users/kooshapari/CodeProjects/Phenotype/repos/AgilePlus && agileplus <command>`

## Key Workflows

1. **Before implementing:** Check AgilePlus for existing specs (`agileplus status`)
2. **Quality gates:** Run linters, tests, and docs validation locally:
   - `cargo test --workspace`
   - `cargo clippy --workspace -- -D warnings`
   - `cargo fmt --check`
3. **Worktrees:** Use `.worktrees/<topic>/` for feature work (e.g., `.worktrees/feature-xyz/`)
4. **Integration:** Commit to canonical repo (`main`) after quality gates pass
5. **Cargo build/check:** Do NOT run cargo build/check in multi-agent sessions (hangs on 70-crate workspace)

## Project-Specific Gotchas

### Build & Testing
- No `cargo build` or `cargo check` in shared sessions (will hang)
- Only `cargo test`, `cargo clippy`, and `cargo fmt` are safe
- See CLAUDE.md for language stack, build commands, and testing requirements

### Workspace Members
- Each crate in `crates/` is independent; check `Cargo.toml` members before adding
- Comment out missing crates with `# missing-as-of-YYYY-MM-DD` rather than removing

### FR Traceability
- All tests MUST reference FR IDs in comments: `// Traces to: FR-XXX-NNN`
- Verify via: `grep -r "Traces to:" crates/*/src/**/*.rs`

---

**Parent contract:** Extends Phenotype-org governance. See `CLAUDE.md` and parent `AGENTS.md` for complete operating procedures.
