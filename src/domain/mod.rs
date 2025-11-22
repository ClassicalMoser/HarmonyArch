use uuid::Uuid;

/// Domain layer for the application
/// Pure domain logic, no external dependencies, no ECS, no Bevy
pub mod primitives;
/// Validation functions for geometry integrity
pub mod validations;

pub use primitives::*;
pub use validations::*;

/// Constant to define unit size for coordinate system
pub const METERS_PER_UNIT: f32 = 1.0;

/// A registry of all geometry objects
pub struct GeometryRegistry {
    /// The vertices in the registry
    pub vertices: VertexRegistry,
    /// The segments in the registry
    pub segments: SegmentRegistry,
    /// The polygons in the registry
    pub polygons: PolygonRegistry,
    /// The solids in the registry
    pub solids: SolidRegistry,
}

impl GeometryRegistry {
    /// Create a new geometry registry
    pub fn create_new() -> Self {
        Self {
            vertices: VertexRegistry::create_new(),
            segments: SegmentRegistry::create_new(),
            polygons: PolygonRegistry::create_new(),
            solids: SolidRegistry::create_new(),
        }
    }
}

/// A tier is a geometry scope
/// This is the basis of the hierarchical geometry system
/// Each tier is propagated to the next tier in a one-way relationship

pub struct Tier {
    /// The name of the tier
    pub name: String,
    /// The geometry associated with the tier
    pub geometry: Vec<Uuid>,
    /// The tolerance of the tier
    pub tolerance: f32,
}
