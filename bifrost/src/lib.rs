//! Bifrost-backed router adapter for OmniRoute v1.5.
//!
//! This crate is the v1.5 pivot target per `D-omni-02` (sign-off at
//! `docs/sessions/20260705-omniroute-backend-rewrite/05-decisions/00-D-OMNI-SIGNOFF.md`).
//! v1 ships with `omni-router` as the placeholder; v1.5 swaps that out for a
//! Bifrost-backed implementation behind the same `RouterPort` trait
//! (defined in `omni-core::executor`).
//!
//! The crate is intentionally thin at scaffold time. It exposes:
//!
//! 1. [`RouterPort`] — the trait contract that any v1/v1.5 router must
//!    satisfy. Today: `InMemoryRouter`, a deterministic router used by tests
//!    and by the v1 placeholder wiring.
//! 2. [`BifrostBackend`] — the v1.5 client stub. It currently fails every
//!    request with [`Error::BackendUnavailable`] so the v1.5 path can be
//!    wired into the dispatch loop without falsely succeeding.
//! 3. [`pick`] — a target-selection strategy that mirrors the v1 fallback
//!    rules in `omni-router` so the v1 → v1.5 cutover is test-equivalent.
//!
//! See: `docs/ROUTING-CONVERGENCE-STATUS.md` § "Tier-1 / Tier-2 Router Split"
//! and the v8.1 Bifrost rollout plan (`PLAN.md` § 2.5.2, items B1-B9).

#![forbid(unsafe_code)]
#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

pub mod backend;
pub mod error;
pub mod router;

pub use backend::BifrostBackend;
pub use error::{Error, Result};
pub use router::{InMemoryRouter, RouteTarget, RouterPort};
