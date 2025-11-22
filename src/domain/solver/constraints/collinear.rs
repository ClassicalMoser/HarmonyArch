/// Collinear constraint
/// 
/// Points/segments must lie on the same line (projection constraint).

use crate::domain::solver::{delta, error, context};
use crate::domain::GeometryRegistry;
use uuid::Uuid;

/// Apply collinear constraint
/// 
/// Ensures that specified vertices/segments are collinear.
/// 
/// # Arguments
/// * `geometry_registry` - Registry containing geometry
/// * `context` - Constraint context
/// * `targets` - Geometry entities that must be collinear
/// 
/// # Returns
/// Deltas to make geometry collinear, or error
pub fn apply_collinear(
    _geometry_registry: &GeometryRegistry,
    _context: &context::TierContext,
    _targets: &[Uuid],
) -> Result<delta::DeltaSet, error::ConstraintError> {
    // TODO: Implement collinear constraint
    // - Use first two points to define line
    // - Project other points onto that line
    // - Create deltas for projected positions
    Ok(delta::DeltaSet::new())
}

