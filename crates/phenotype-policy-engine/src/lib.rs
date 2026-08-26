//! Generic policy evaluation engine for Phenotype.

pub mod context;
pub mod engine;
pub mod error;
pub mod loader;
pub mod policy;
pub mod result;
pub mod rule;

#[cfg(feature = "casbin-backend")]
pub mod casbin_backend;

#[cfg(feature = "casbin-backend")]
pub use casbin_backend::{
    CasbinBackend, CasbinBackendError, CasbinRequest, CasbinResponse, PolicyBackend,
};
pub use context::EvaluationContext;
pub use engine::PolicyEngine;
pub use error::PolicyEngineError;
pub use policy::Policy;
pub use result::{PolicyResult, Severity, Violation};
pub use rule::{Rule, RuleType};

pub mod prelude {
    pub use crate::context::EvaluationContext;
    pub use crate::engine::PolicyEngine;
    pub use crate::error::PolicyEngineError;
    pub use crate::policy::Policy;
    pub use crate::result::{PolicyResult, Severity, Violation};
    pub use crate::rule::{Rule, RuleType};
}
