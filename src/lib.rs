//! HarmonyArch - Semantic architectural modeling engine
//!
//! A domain-driven design for architectural modeling with hexagonal architecture.

pub mod application;
pub mod composition;
pub mod domain;
pub mod infrastructure;

// Re-export commonly used domain types
pub use application::GeometryService;
pub use domain::{Element, ElementType, Point, Vector};
pub use infrastructure::StlRenderer;
