/// Vertical constraint
/// Ensures lines, edges, or surfaces are perpendicular to the ground plane
use super::{Constraint, GeometryState, JacobianRow};
use std::any::Any;

/// Vertical constraint between two points
/// Ensures the line between two points is vertical (perpendicular to ground)
#[derive(Debug, Clone)]
pub struct VerticalConstraint {
    /// Index of first point in geometry state
    pub point_a_idx: usize,
    /// Index of second point in geometry state
    pub point_b_idx: usize,
    /// Constraint priority
    pub priority: u32,
}

impl Constraint for VerticalConstraint {
    fn constraint_type(&self) -> &'static str {
        "Vertical"
    }

    fn residual(&self, geometry: &GeometryState) -> f32 {
        let point_a = geometry.get_point(self.point_a_idx).unwrap();
        let point_b = geometry.get_point(self.point_b_idx).unwrap();

        // Calculate the horizontal distance (should be 0 for vertical)
        let dx = point_b.x - point_a.x;
        let dy = point_b.y - point_a.y;
        let horizontal_distance = (dx * dx + dy * dy).sqrt();

        horizontal_distance
    }

    fn jacobian_row(&self, _geometry: &GeometryState) -> JacobianRow {
        // TODO: Implement proper derivatives for vertical constraints
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

/// Vertical alignment constraint for a single point
/// Ensures a point is directly above/below another point
#[derive(Debug, Clone)]
pub struct VerticalAlignmentConstraint {
    /// Index of the base point (reference)
    pub base_point_idx: usize,
    /// Index of the point to align vertically
    pub aligned_point_idx: usize,
    /// Constraint priority
    pub priority: u32,
}

impl Constraint for VerticalAlignmentConstraint {
    fn constraint_type(&self) -> &'static str {
        "VerticalAlignment"
    }

    fn residual(&self, geometry: &GeometryState) -> f32 {
        let base_point = geometry.get_point(self.base_point_idx).unwrap();
        let aligned_point = geometry.get_point(self.aligned_point_idx).unwrap();

        // Calculate the horizontal distance (should be 0 for vertical alignment)
        let dx = aligned_point.x - base_point.x;
        let dy = aligned_point.y - base_point.y;
        let horizontal_distance = (dx * dx + dy * dy).sqrt();

        horizontal_distance
    }

    fn jacobian_row(&self, _geometry: &GeometryState) -> JacobianRow {
        // TODO: Implement proper derivatives for vertical alignment constraints
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
    fn test_vertical_constraint_residual() {
        let mut geometry = GeometryState::new();
        let point_a_idx = geometry.add_point(Point {
            x: 5.0,
            y: 5.0,
            z: 0.0,
        });
        let point_b_idx = geometry.add_point(Point {
            x: 5.0,
            y: 5.0,
            z: 10.0,
        });

        let constraint = VerticalConstraint {
            point_a_idx,
            point_b_idx,
            priority: 1,
        };

        // Should be satisfied (same x,y coordinates)
        assert!(constraint.residual(&geometry) < 1e-6);

        // Change point B to violate constraint
        geometry.points[point_b_idx] = Point {
            x: 5.1,
            y: 5.0,
            z: 10.0,
        };
        assert!(constraint.residual(&geometry) > 0.0);
    }

    #[test]
    fn test_vertical_alignment_constraint_residual() {
        let mut geometry = GeometryState::new();
        let base_idx = geometry.add_point(Point {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        });
        let aligned_idx = geometry.add_point(Point {
            x: 0.0,
            y: 0.0,
            z: 5.0,
        });

        let constraint = VerticalAlignmentConstraint {
            base_point_idx: base_idx,
            aligned_point_idx: aligned_idx,
            priority: 1,
        };

        // Should be satisfied (aligned vertically)
        assert!(constraint.residual(&geometry) < 1e-6);

        // Change aligned point to violate constraint
        geometry.points[aligned_idx] = Point {
            x: 0.1,
            y: 0.0,
            z: 5.0,
        };
        assert!(constraint.residual(&geometry) > 0.0);
    }
}
