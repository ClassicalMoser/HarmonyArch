//! `HarmonyArch` - Semantic architectural modeling engine
//!
//! A domain-driven design for architectural modeling with hexagonal architecture.
//! 

#![deny(warnings)]
// #![deny(unsafe_code)]  // Temporarily disabled for OpenGL integration
#![deny(missing_docs)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]


/// Application layer for the application
pub mod application;
/// Composition layer for the application
pub mod composition;
/// Domain layer for the application
pub mod domain;
/// Infrastructure layer for the application
pub mod infrastructure;
/// Interface layer for the application
pub mod interface;
