/// Core types for the constraint system
///
/// Defines the fundamental constraint types, kinds, and structures
/// that the constraint solver operates on.
use uuid::Uuid;

/// The kind of constraint being applied
///
/// Matches the ordering defined in ORDER.md
#[derive(Debug, Clone, PartialEq)]
pub enum ConstraintKind {
    /// Coincident (Rare) - Points must be at the same location
    Coincident,
    /// Collinear (Projection) - Points/segments must lie on the same line
    Collinear,
    /// Coplanar (Projection) - Points/segments/polygons must lie on the same plane
    Coplanar,
    /// Boundary - Geometry must respect parent tier boundary
    Boundary,
    /// Equilateral - Segments must have equal length
    Equilateral,
    /// Equiangular - Angles must be equal
    Equiangular,
    /// Plumb (Opt-out) - Vertical alignment (default enabled)
    Plumb,
    /// Level (Opt-out) - Horizontal alignment (default enabled)
    Level,
    /// Orthogonal (Opt-out) - Right-angle alignment (default enabled)
    Orthogonal,
}

/// Reference for relational constraints
///
/// Some constraints (coplanar, orthogonal) need to reference other geometry
/// or define a reference frame. This enum captures those references.
#[derive(Debug, Clone)]
pub enum ConstraintReference {
    /// Self-referential: the constraint set defines its own reference
    /// (e.g., first 3 vertices define plane for coplanar)
    SelfDefined,
    // TODO: Add explicit reference types as needed
    // Plane(PlaneRef),
    // Axis(AxisRef),
    // Direction(Vector),
}

/// A single constraint assignment
///
/// Defines a constraint applied to specific geometry entities.
#[derive(Debug, Clone)]
pub struct Constraint {
    /// The kind of constraint
    pub kind: ConstraintKind,
    /// The geometry entities this constraint applies to
    pub targets: Vec<Uuid>,
    /// Optional reference for relational constraints
    pub reference: Option<ConstraintReference>,
}

/// Configuration for opt-out constraints
///
/// Architectural constraints (plumb, level, orthogonal) are opt-out.
/// This struct tracks which ones are enabled (default true).
#[derive(Debug, Clone)]
pub struct OptOutConstraints {
    /// Plumb constraint enabled (default: true)
    pub plumb_enabled: bool,
    /// Level constraint enabled (default: true)
    pub level_enabled: bool,
    /// Orthogonal constraint enabled (default: true)
    pub orthogonal_enabled: bool,
}

impl Default for OptOutConstraints {
    fn default() -> Self {
        Self {
            plumb_enabled: true,
            level_enabled: true,
            orthogonal_enabled: true,
        }
    }
}

/// A set of constraints for a tier
///
/// Combines opt-out constraint flags with explicit constraint assignments.
#[derive(Debug, Clone, Default)]
pub struct ConstraintSet {
    /// Opt-out constraint configuration
    pub opt_out: OptOutConstraints,
    /// Explicit constraint assignments
    pub explicit: Vec<Constraint>,
}
