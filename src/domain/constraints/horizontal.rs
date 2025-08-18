/// Horizontal constraint
/// Ensures lines, edges, or surfaces are parallel to the ground plane
use super::{Constraint, GeometryState, JacobianRow};
use std::any::Any;

/// Horizontal constraint between two points
/// Ensures the line between two points is horizontal (parallel to ground)
#[derive(Debug, Clone)]
pub struct HorizontalConstraint {
    /// Index of first point in geometry state
    pub point_a_idx: usize,
    /// Index of second point in geometry state
    pub point_b_idx: usize,
    /// Constraint priority
    pub priority: u32,
}

impl Constraint for HorizontalConstraint {
    fn constraint_type(&self) -> &'static str {
        "Horizontal"
    }

    fn residual(&self, geometry: &GeometryState) -> f32 {
        let point_a = geometry.get_point(self.point_a_idx).unwrap();
        let point_b = geometry.get_point(self.point_b_idx).unwrap();

        // Calculate the height difference (should be 0 for horizontal)
        let height_diff = (point_b.z - point_a.z).abs();
        height_diff
    }

    fn jacobian_row(&self, _geometry: &GeometryState) -> JacobianRow {
        // TODO: Implement proper derivatives for horizontal constraints
        // For now, return a simple approximation
        JacobianRow {
            constraint_idx: 0,
            derivatives: vec![],
        }
    }

    fn priority(&self) -> u32 {
        self.priority
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Horizontal constraint for a single point relative to a reference height
/// Ensures a point is at a specific height (horizontal plane)
#[derive(Debug, Clone)]
pub struct HeightConstraint {
    /// Index of the point in geometry state
    pub point_idx: usize,
    /// The required height (z-coordinate)
    pub required_height: f32,
    /// Constraint priority
    pub priority: u32,
}

impl Constraint for HeightConstraint {
    fn constraint_type(&self) -> &'static str {
        "Height"
    }

    fn residual(&self, geometry: &GeometryState) -> f32 {
        let point = geometry.get_point(self.point_idx).unwrap();

        // Calculate the height difference from required height
        let height_diff = (point.z - self.required_height).abs();
        height_diff
    }

    fn jacobian_row(&self, _geometry: &GeometryState) -> JacobianRow {
        // TODO: Implement proper derivatives for height constraints
        JacobianRow {
            constraint_idx: 0,
            derivatives: vec![],
        }
    }

    fn priority(&self) -> u32 {
        self.priority
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::Point;

    #[test]
    fn test_horizontal_constraint_residual() {
        let mut geometry = GeometryState::new();
        let point_a_idx = geometry.add_point(Point {
            x: 0.0,
            y: 0.0,
            z: 5.0,
        });
        let point_b_idx = geometry.add_point(Point {
            x: 10.0,
            y: 0.0,
            z: 5.0,
        });

        let constraint = HorizontalConstraint {
            point_a_idx,
            point_b_idx,
            priority: 1,
        };

        // Should be satisfied (same height)
        assert!(constraint.residual(&geometry) < 1e-6);

        // Change point B height to violate constraint
        geometry.points[point_b_idx] = Point {
            x: 10.0,
            y: 0.0,
            z: 5.1,
        };
        assert!(constraint.residual(&geometry) > 0.0);
    }

    #[test]
    fn test_height_constraint_residual() {
        let mut geometry = GeometryState::new();
        let point_idx = geometry.add_point(Point {
            x: 0.0,
            y: 0.0,
            z: 3.0,
        });

        let constraint = HeightConstraint {
            point_idx,
            required_height: 3.0,
            priority: 1,
        };

        // Should be satisfied (correct height)
        assert!(constraint.residual(&geometry) < 1e-6);

        // Change point height to violate constraint
        geometry.points[point_idx] = Point {
            x: 0.0,
            y: 0.0,
            z: 3.1,
        };
        assert!(constraint.residual(&geometry) > 0.0);
    }
}
