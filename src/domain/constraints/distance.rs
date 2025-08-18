/// Distance constraint between two points
/// Ensures two points maintain a specific distance relationship
use super::{Constraint, GeometryState, JacobianRow};
use std::any::Any;

/// Distance constraint between two points
#[derive(Debug, Clone)]
pub struct DistanceConstraint {
    /// Index of first point in geometry state
    pub point_a_idx: usize,
    /// Index of second point in geometry state
    pub point_b_idx: usize,
    /// The required distance between points
    pub required_distance: f32,
    /// Constraint priority (lower = higher priority)
    pub priority: u32,
}

impl Constraint for DistanceConstraint {
    fn constraint_type(&self) -> &'static str {
        "Distance"
    }

    fn residual(&self, geometry: &GeometryState) -> f32 {
        let point_a = geometry.get_point(self.point_a_idx).unwrap();
        let point_b = geometry.get_point(self.point_b_idx).unwrap();

        let vector = crate::domain::measure_vector(point_a, point_b);
        let actual_distance =
            (vector.x * vector.x + vector.y * vector.y + vector.z * vector.z).sqrt();
        (actual_distance - self.required_distance).abs()
    }

    fn jacobian_row(&self, geometry: &GeometryState) -> JacobianRow {
        let point_a = geometry.get_point(self.point_a_idx).unwrap();
        let point_b = geometry.get_point(self.point_b_idx).unwrap();

        let dx = point_b.x - point_a.x;
        let dy = point_b.y - point_a.y;
        let dz = point_b.z - point_a.z;
        let distance = (dx * dx + dy * dy + dz * dz).sqrt();

        if distance == 0.0 {
            // Degenerate case - return zero derivatives
            return JacobianRow {
                constraint_idx: 0,
                derivatives: vec![],
            };
        }

        // Normalize the direction vector
        let nx = dx / distance;
        let ny = dy / distance;
        let nz = dz / distance;

        // Derivatives: ∂d/∂x = nx, ∂d/∂y = ny, ∂d/∂z = nz
        let mut derivatives = Vec::new();

        // Point A derivatives (negative because moving A away from B decreases distance)
        derivatives.push((self.point_a_idx * 3, -nx));
        derivatives.push((self.point_a_idx * 3 + 1, -ny));
        derivatives.push((self.point_a_idx * 3 + 2, -nz));

        // Point B derivatives (positive because moving B away from A increases distance)
        derivatives.push((self.point_b_idx * 3, nx));
        derivatives.push((self.point_b_idx * 3 + 1, ny));
        derivatives.push((self.point_b_idx * 3 + 2, nz));

        JacobianRow {
            constraint_idx: 0,
            derivatives,
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
    fn test_distance_constraint_residual() {
        let mut geometry = GeometryState::new();
        let point_a_idx = geometry.add_point(Point {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        });
        let point_b_idx = geometry.add_point(Point {
            x: 3.0,
            y: 4.0,
            z: 0.0,
        });

        let constraint = DistanceConstraint {
            point_a_idx,
            point_b_idx,
            required_distance: 5.0,
            priority: 1,
        };

        // Should be satisfied (3-4-5 triangle)
        assert!(constraint.residual(&geometry) < 1e-6);

        // Change point B to violate constraint
        geometry.points[point_b_idx] = Point {
            x: 3.0,
            y: 4.1,
            z: 0.0,
        };
        assert!(constraint.residual(&geometry) > 0.0);
    }
}
