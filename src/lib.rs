//! HarmonyArch - Semantic architectural modeling engine
//!
//! A domain-driven design for architectural modeling with hexagonal architecture.

pub mod domain;
pub mod application;
pub mod infrastructure;
pub mod composition;

// Re-export commonly used domain types
pub use domain::{Point, Vector, Element, ElementType};
pub use application::GeometryService;
pub use infrastructure::StlRenderer;
