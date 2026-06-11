# pheno SSOT

Last updated: 2026-06-11

This document is the source-of-truth index for pheno architecture, ADRs, specifications, governance, audits, synthesis reports, and execution plans. It does not replace the linked documents; it gives maintainers one stable entry point into them.

## Table of Contents

- [Canonical Roots](#canonical-roots)
- [Architecture](#architecture)
- [ADRs and Governance Decisions](#adrs-and-governance-decisions)
- [Governance](#governance)
- [Specifications and PRDs](#specifications-and-prds)
- [Plans and Work Breakdown](#plans-and-work-breakdown)
- [Audits and Synthesis](#audits-and-synthesis)
- [SSOT and Reference Indexes](#ssot-and-reference-indexes)
- [Worklogs and Session Sources](#worklogs-and-session-sources)

## Canonical Roots

| Source | Last updated |
| --- | --- |
| [GOVERNANCE.md](../GOVERNANCE.md) | 2026-04-30 |
| [PRD.md](../PRD.md) | 2026-04-25 |
| [PLAN.md](../PLAN.md) | 2026-03-30 |
| [PLAN_REGISTRY.md](../PLAN_REGISTRY.md) | 2026-03-31 |
| [CI_REMEDIATION_PLAN.md](../CI_REMEDIATION_PLAN.md) | 2026-03-30 |
| [CONSOLIDATION_AUDIT.md](../CONSOLIDATION_AUDIT.md) | 2026-03-30 |
| [DUPLICATION_AUDIT.md](../DUPLICATION_AUDIT.md) | 2026-03-29 |
| [PHENOTYPE_AUDIT_REPORT.md](../PHENOTYPE_AUDIT_REPORT.md) | 2026-04-02 |
| [PHENOTYPE_TEST_AUDIT.md](../PHENOTYPE_TEST_AUDIT.md) | 2026-04-02 |

## Architecture

| Source | Last updated |
| --- | --- |
| [docs/architecture.md](architecture.md) | 2026-03-24 |
| [docs/architecture/overview.md](architecture/overview.md) | 2026-03-30 |
| [docs/architecture/domain-model.md](architecture/domain-model.md) | 2026-03-30 |
| [docs/architecture/ports.md](architecture/ports.md) | 2026-03-30 |
| [docs/adr/0001-record-architecture-decisions.md](adr/0001-record-architecture-decisions.md) | 2026-04-27 |
| [docs/adr/002-registry-adapter-architecture.md](adr/002-registry-adapter-architecture.md) | 2026-03-30 |
| [docs/adr/ADR-012-plugin-architecture.md](adr/ADR-012-plugin-architecture.md) | 2026-03-30 |
| [docs/adr/ARCHITECTURE.md](adr/ARCHITECTURE.md) | 2026-03-30 |
| [docs/reference/SSOT_ARCHITECTURE_INDEX.md](reference/SSOT_ARCHITECTURE_INDEX.md) | 2026-03-31 |
| [docs/reference/POLYREPO_SSOT_ARCHITECTURE.md](reference/POLYREPO_SSOT_ARCHITECTURE.md) | 2026-03-31 |
| [docs/worklogs/ARCHITECTURE.md](worklogs/ARCHITECTURE.md) | 2026-04-30 |

## ADRs and Governance Decisions

| Source | Last updated |
| --- | --- |
| [docs/adr/0001-record-architecture-decisions.md](adr/0001-record-architecture-decisions.md) | 2026-04-27 |
| [docs/adr/001-task-runner-selection.md](adr/001-task-runner-selection.md) | 2026-03-30 |
| [docs/adr/002-registry-adapter-architecture.md](adr/002-registry-adapter-architecture.md) | 2026-03-30 |
| [docs/adr/ADR-012-plugin-architecture.md](adr/ADR-012-plugin-architecture.md) | 2026-03-30 |
| [docs/adr/ADR-015-crate-organization.md](adr/ADR-015-crate-organization.md) | 2026-03-30 |
| [docs/adr/ARCHITECTURE.md](adr/ARCHITECTURE.md) | 2026-03-30 |
| [docs/governance/ADR-001-external-package-adoption.md](governance/ADR-001-external-package-adoption.md) | 2026-03-29 |
| [docs/governance/ADR-002-event-sourcing-strategy.md](governance/ADR-002-event-sourcing-strategy.md) | 2026-03-30 |
| [docs/governance/ADR-003-microservices-coordination.md](governance/ADR-003-microservices-coordination.md) | 2026-03-30 |

## Governance

| Source | Last updated |
| --- | --- |
| [GOVERNANCE.md](../GOVERNANCE.md) | 2026-04-30 |
| [docs/governance/ADR-001-external-package-adoption.md](governance/ADR-001-external-package-adoption.md) | 2026-03-29 |
| [docs/governance/ADR-002-event-sourcing-strategy.md](governance/ADR-002-event-sourcing-strategy.md) | 2026-03-30 |
| [docs/governance/ADR-003-microservices-coordination.md](governance/ADR-003-microservices-coordination.md) | 2026-03-30 |
| [docs/concepts/governance.md](concepts/governance.md) | 2026-03-30 |
| [docs/process/governance.md](process/governance.md) | 2026-03-30 |
| [docs/process/constitution.md](process/constitution.md) | 2026-03-30 |
| [docs/specs/002-org-wide-release-governance-dx-automation/spec.md](specs/002-org-wide-release-governance-dx-automation/spec.md) | 2026-03-30 |
| [docs/specs/002-org-wide-release-governance-dx-automation/plan.md](specs/002-org-wide-release-governance-dx-automation/plan.md) | 2026-03-30 |
| [docs/specs/002-org-wide-release-governance-dx-automation/tasks.md](specs/002-org-wide-release-governance-dx-automation/tasks.md) | 2026-03-30 |
| [docs/worklogs/GOVERNANCE.md](worklogs/GOVERNANCE.md) | 2026-04-30 |
| [agileplus/GOVERNANCE.md](../agileplus/GOVERNANCE.md) | 2026-04-30 |

## Specifications and PRDs

| Source | Last updated |
| --- | --- |
| [PRD.md](../PRD.md) | 2026-04-25 |
| [docs/specs/001-spec-driven-development-engine/spec.md](specs/001-spec-driven-development-engine/spec.md) | 2026-03-30 |
| [docs/specs/002-org-wide-release-governance-dx-automation/spec.md](specs/002-org-wide-release-governance-dx-automation/spec.md) | 2026-03-30 |
| [docs/specs/003-agileplus-platform-completion/spec.md](specs/003-agileplus-platform-completion/spec.md) | 2026-03-30 |
| [docs/specs/004-modules-and-cycles/spec.md](specs/004-modules-and-cycles/spec.md) | 2026-03-30 |
| [docs/specs/005-heliosapp-completion/spec.md](specs/005-heliosapp-completion/spec.md) | 2026-03-30 |
| [docs/specs/006-helioscli-completion/spec.md](specs/006-helioscli-completion/spec.md) | 2026-03-30 |
| [docs/specs/007-thegent-completion/spec.md](specs/007-thegent-completion/spec.md) | 2026-03-30 |
| [docs/reference/RETROSPECTIVE_ANALYSIS_INDEX.md](reference/RETROSPECTIVE_ANALYSIS_INDEX.md) | 2026-03-30 |
| [python/SPEC.md](../python/SPEC.md) | 2026-04-02 |
| [template-python/SPEC.md](../template-python/SPEC.md) | 2026-04-02 |
| [template-rust/SPEC.md](../template-rust/SPEC.md) | 2026-04-02 |
| [agileplus/SPEC.md](../agileplus/SPEC.md) | 2026-04-02 |
| [agileplus/PRD.md](../agileplus/PRD.md) | 2026-03-30 |
| [agileplus-agents/SPEC.md](../agileplus-agents/SPEC.md) | 2026-04-02 |
| [agileplus-mcp/SPEC.md](../agileplus-mcp/SPEC.md) | 2026-04-02 |
| [forgecode-fork/SPEC.md](../forgecode-fork/SPEC.md) | 2026-04-02 |
| [phenotype-infrakit/SPEC.md](../phenotype-infrakit/SPEC.md) | 2026-04-02 |
| [phenotype-router-monitor/SPEC.md](../phenotype-router-monitor/SPEC.md) | 2026-04-02 |

## Plans and Work Breakdown

| Source | Last updated |
| --- | --- |
| [PLAN.md](../PLAN.md) | 2026-03-30 |
| [PLAN_REGISTRY.md](../PLAN_REGISTRY.md) | 2026-03-31 |
| [CI_REMEDIATION_PLAN.md](../CI_REMEDIATION_PLAN.md) | 2026-03-30 |
| [docs/PLAN.md](PLAN.md) | 2026-04-02 |
| [docs/specs/001-spec-driven-development-engine/plan.md](specs/001-spec-driven-development-engine/plan.md) | 2026-03-30 |
| [docs/specs/001-spec-driven-development-engine/tasks.md](specs/001-spec-driven-development-engine/tasks.md) | 2026-03-30 |
| [docs/specs/002-org-wide-release-governance-dx-automation/plan.md](specs/002-org-wide-release-governance-dx-automation/plan.md) | 2026-03-30 |
| [docs/specs/002-org-wide-release-governance-dx-automation/tasks.md](specs/002-org-wide-release-governance-dx-automation/tasks.md) | 2026-03-30 |
| [docs/specs/003-agileplus-platform-completion/plan.md](specs/003-agileplus-platform-completion/plan.md) | 2026-03-30 |
| [docs/specs/003-agileplus-platform-completion/tasks.md](specs/003-agileplus-platform-completion/tasks.md) | 2026-03-30 |
| [docs/specs/004-modules-and-cycles/plan.md](specs/004-modules-and-cycles/plan.md) | 2026-03-30 |
| [docs/specs/004-modules-and-cycles/tasks.md](specs/004-modules-and-cycles/tasks.md) | 2026-03-30 |
| [docs/specs/005-heliosapp-completion/plan.md](specs/005-heliosapp-completion/plan.md) | 2026-03-30 |
| [docs/specs/005-heliosapp-completion/tasks.md](specs/005-heliosapp-completion/tasks.md) | 2026-03-30 |
| [docs/specs/007-thegent-completion/plan.md](specs/007-thegent-completion/plan.md) | 2026-03-30 |
| [docs/audits/CONFIG_MIGRATION_PLAN.md](audits/CONFIG_MIGRATION_PLAN.md) | 2026-03-30 |
| [docs/worklogs/PLANS/ImplementationPlanDuplication.md](worklogs/PLANS/ImplementationPlanDuplication.md) | 2026-03-30 |
| [agileplus/PLAN.md](../agileplus/PLAN.md) | 2026-03-30 |

## Audits and Synthesis

No `2026-06-*` audit, synthesis, or plan markdown files were present when this index was refreshed.

| Source | Last updated |
| --- | --- |
| [CONSOLIDATION_AUDIT.md](../CONSOLIDATION_AUDIT.md) | 2026-03-30 |
| [DUPLICATION_AUDIT.md](../DUPLICATION_AUDIT.md) | 2026-03-29 |
| [PHENOTYPE_AUDIT_REPORT.md](../PHENOTYPE_AUDIT_REPORT.md) | 2026-04-02 |
| [PHENOTYPE_TEST_AUDIT.md](../PHENOTYPE_TEST_AUDIT.md) | 2026-04-02 |
| [docs/audits/INDEX.md](audits/INDEX.md) | 2026-03-30 |
| [docs/audits/2026-03-30-agent-wave-audit.md](audits/2026-03-30-agent-wave-audit.md) | 2026-03-30 |
| [docs/audits/2026-03-30-cliproxyapi-plusplus-audit.md](audits/2026-03-30-cliproxyapi-plusplus-audit.md) | 2026-03-30 |
| [docs/audits/2026-03-30-heliosCLI-audit.md](audits/2026-03-30-heliosCLI-audit.md) | 2026-03-30 |
| [docs/audits/2026-03-30-root-workspace-audit.md](audits/2026-03-30-root-workspace-audit.md) | 2026-03-30 |
| [docs/audits/CARGO_WORKSPACE_AUDIT_2026-03-30.md](audits/CARGO_WORKSPACE_AUDIT_2026-03-30.md) | 2026-03-30 |
| [docs/audits/CONFIG_CONSOLIDATION_AUDIT.md](audits/CONFIG_CONSOLIDATION_AUDIT.md) | 2026-03-30 |
| [docs/audits/CONFIG_MIGRATION_PLAN.md](audits/CONFIG_MIGRATION_PLAN.md) | 2026-03-30 |
| [docs/audits/VIBEPROXY_ROUTING_AUDIT_2026-03-30.md](audits/VIBEPROXY_ROUTING_AUDIT_2026-03-30.md) | 2026-03-30 |
| [docs/audits/WORKSPACE_ORPHANS_AND_STALE_2026-03-30.md](audits/WORKSPACE_ORPHANS_AND_STALE_2026-03-30.md) | 2026-03-30 |
| [docs/reports/MASTER_DUPLICATION_AUDIT.md](reports/MASTER_DUPLICATION_AUDIT.md) | 2026-03-29 |
| [docs/reports/CROSS_PROJECT_DUPLICATION_ANALYSIS.md](reports/CROSS_PROJECT_DUPLICATION_ANALYSIS.md) | 2026-03-29 |
| [docs/reports/DECOMPOSITION_AUDIT.md](reports/DECOMPOSITION_AUDIT.md) | 2026-03-30 |
| [docs/reports/WEEK2_DEPENDENCY_OPTIMIZATION_REPORT.md](reports/WEEK2_DEPENDENCY_OPTIMIZATION_REPORT.md) | 2026-03-30 |
| [docs/reference/PHENOSDK_ANALYSIS_INDEX.md](reference/PHENOSDK_ANALYSIS_INDEX.md) | 2026-03-30 |
| [docs/reference/RETROSPECTIVE_ANALYSIS_INDEX.md](reference/RETROSPECTIVE_ANALYSIS_INDEX.md) | 2026-03-30 |
| [docs/reference/RUST_DEPENDENCY_ANALYSIS_INDEX.md](reference/RUST_DEPENDENCY_ANALYSIS_INDEX.md) | 2026-03-30 |
| [docs/worklogs/AUDIT_SUMMARY_2026-03-30.md](worklogs/AUDIT_SUMMARY_2026-03-30.md) | 2026-03-30 |
| [docs/worklogs/BUILD_PERFORMANCE_AUDIT_2026-03-30.md](worklogs/BUILD_PERFORMANCE_AUDIT_2026-03-30.md) | 2026-03-30 |
| [docs/worklogs/DEAD_CODE_CLEANUP_AUDIT_2026-03-30.md](worklogs/DEAD_CODE_CLEANUP_AUDIT_2026-03-30.md) | 2026-03-30 |
| [docs/worklogs/EXECUTIVE_SUMMARY_SESSION_2026-03-30.md](worklogs/EXECUTIVE_SUMMARY_SESSION_2026-03-30.md) | 2026-03-30 |

## SSOT and Reference Indexes

| Source | Last updated |
| --- | --- |
| [docs/audits/INDEX.md](audits/INDEX.md) | 2026-03-30 |
| [docs/reference/SSOT_ARCHITECTURE_INDEX.md](reference/SSOT_ARCHITECTURE_INDEX.md) | 2026-03-31 |
| [docs/reference/SSOT_IMPLEMENTATION_ROADMAP.md](reference/SSOT_IMPLEMENTATION_ROADMAP.md) | 2026-04-27 |
| [docs/reference/SSOT_PHASE1_MASTER_INDEX.md](reference/SSOT_PHASE1_MASTER_INDEX.md) | 2026-03-31 |
| [docs/reference/SSOT_QUICK_REFERENCE.md](reference/SSOT_QUICK_REFERENCE.md) | 2026-03-31 |
| [docs/reference/POLYREPO_SSOT_ARCHITECTURE.md](reference/POLYREPO_SSOT_ARCHITECTURE.md) | 2026-03-31 |
| [docs/reference/TRACEABILITY_MAP.md](reference/TRACEABILITY_MAP.md) | 2026-03-30 |
| [docs/reference/QUALITY_GATES_INDEX.md](reference/QUALITY_GATES_INDEX.md) | 2026-03-31 |
| [docs/reference/DEPENDENCY_PHASE2_INDEX.md](reference/DEPENDENCY_PHASE2_INDEX.md) | 2026-03-31 |
| [docs/reference/PHENOSDK_ANALYSIS_INDEX.md](reference/PHENOSDK_ANALYSIS_INDEX.md) | 2026-03-30 |
| [docs/reference/RETROSPECTIVE_ANALYSIS_INDEX.md](reference/RETROSPECTIVE_ANALYSIS_INDEX.md) | 2026-03-30 |
| [docs/reference/RUST_DEPENDENCY_ANALYSIS_INDEX.md](reference/RUST_DEPENDENCY_ANALYSIS_INDEX.md) | 2026-03-30 |
| [docs/reference/SAST_DEPLOYMENT_INDEX.md](reference/SAST_DEPLOYMENT_INDEX.md) | 2026-03-31 |
| [docs/reference/SENTRY_IMPLEMENTATION_INDEX.md](reference/SENTRY_IMPLEMENTATION_INDEX.md) | 2026-03-31 |

## Worklogs and Session Sources

| Source | Last updated |
| --- | --- |
| [docs/worklogs/ARCHITECTURE.md](worklogs/ARCHITECTURE.md) | 2026-04-30 |
| [docs/worklogs/GOVERNANCE.md](worklogs/GOVERNANCE.md) | 2026-04-30 |
| [docs/worklogs/AUDIT_SUMMARY_2026-03-30.md](worklogs/AUDIT_SUMMARY_2026-03-30.md) | 2026-03-30 |
| [docs/worklogs/BUILD_PERFORMANCE_AUDIT_2026-03-30.md](worklogs/BUILD_PERFORMANCE_AUDIT_2026-03-30.md) | 2026-03-30 |
| [docs/worklogs/DEAD_CODE_CLEANUP_AUDIT_2026-03-30.md](worklogs/DEAD_CODE_CLEANUP_AUDIT_2026-03-30.md) | 2026-03-30 |
| [docs/worklogs/EXECUTIVE_SUMMARY_SESSION_2026-03-30.md](worklogs/EXECUTIVE_SUMMARY_SESSION_2026-03-30.md) | 2026-03-30 |
| [docs/worklogs/PR_REVIEW_SESSION_2026-03-30.md](worklogs/PR_REVIEW_SESSION_2026-03-30.md) | 2026-03-31 |
| [docs/worklogs/SESSION_2026-03-30_COMPREHENSIVE.md](worklogs/SESSION_2026-03-30_COMPREHENSIVE.md) | 2026-03-30 |
| [docs/worklogs/SESSION_COMPLETION_FINAL_2026-03-30.md](worklogs/SESSION_COMPLETION_FINAL_2026-03-30.md) | 2026-03-30 |
| [docs/worklogs/PLANS/ImplementationPlanDuplication.md](worklogs/PLANS/ImplementationPlanDuplication.md) | 2026-03-30 |

