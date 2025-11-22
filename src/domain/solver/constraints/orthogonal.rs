/// Orthogonal constraint (Opt-out)
/// 
/// Right-angle alignment. Applied by default unless explicitly disabled.

use crate::domain::solver::{delta, error, context};
use crate::domain::GeometryRegistry;
use uuid::Uuid;

/// Apply orthogonal constraint
/// 
/// Ensures that specified geometry forms right angles.
/// 
/// # Arguments
/// * `geometry_registry` - Registry containing geometry
/// * `context` - Constraint context (includes opt-out flag)
/// * `targets` - Geometry entities that must be orthogonal
/// 
/// # Returns
/// Deltas to make geometry orthogonal, or error
pub fn apply_orthogonal(
    _geometry_registry: &GeometryRegistry,
    _context: &context::TierContext,
    _targets: &[Uuid],
) -> Result<delta::DeltaSet, error::ConstraintError> {
    // Check if orthogonal is enabled (opt-out constraint)
    if !_context.constraints.opt_out.orthogonal_enabled {
        return Ok(delta::DeltaSet::new());
    }

    // TODO: Implement orthogonal constraint
    // - For segments: ensure they form right angles
    // - For polygons: ensure edges are perpendicular
    // - Create deltas for adjusted positions
    Ok(delta::DeltaSet::new())
}

