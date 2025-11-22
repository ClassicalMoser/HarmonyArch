/// Plumb constraint (Opt-out)
/// 
/// Vertical alignment relative to world Y-axis (gravity).
/// Applied by default unless explicitly disabled.

use crate::domain::solver::{delta, error, context};
use crate::domain::GeometryRegistry;
use uuid::Uuid;

/// Apply plumb constraint
/// 
/// Ensures that specified geometry is vertical (plumb).
/// 
/// # Arguments
/// * `geometry_registry` - Registry containing geometry
/// * `context` - Constraint context (includes opt-out flag)
/// * `targets` - Geometry entities that must be plumb
/// 
/// # Returns
/// Deltas to make geometry plumb, or error
pub fn apply_plumb(
    _geometry_registry: &GeometryRegistry,
    _context: &context::TierContext,
    _targets: &[Uuid],
) -> Result<delta::DeltaSet, error::ConstraintError> {
    // Check if plumb is enabled (opt-out constraint)
    if !_context.constraints.opt_out.plumb_enabled {
        return Ok(delta::DeltaSet::new());
    }

    // TODO: Implement plumb constraint
    // - For segments: ensure they are vertical (parallel to Y-axis)
    // - For vertices: align X and Z coordinates to reference
    // - Create deltas for adjusted positions
    Ok(delta::DeltaSet::new())
}

