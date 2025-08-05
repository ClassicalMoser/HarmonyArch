//! `HarmonyArch` - Semantic architectural modeling engine
//!
//! A domain-driven design for architectural modeling with hexagonal architecture.
//! 

#![deny(warnings)]
// #![deny(unsafe_code)]  // Temporarily disabled for OpenGL integration
#![deny(missing_docs)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]


pub mod application;
pub mod composition;
pub mod domain;
pub mod infrastructure;
pub mod interface;

// Re-export commonly used domain types
pub use application::GeometryService;
pub use domain::{Element, ElementType, Point};
pub use infrastructure::StlRenderer;
