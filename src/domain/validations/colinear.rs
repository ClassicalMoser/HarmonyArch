/// Validate that vertices are collinear
///
/// This function checks if the given vertices form a degenerate triangle
/// (i.e., they have zero area). For 3 or fewer vertices, they are always
/// considered collinear.
///
/// # Mathematical Approach
/// - For less than 3 vertices, they are always collinear
/// - For 3 vertices, we check if the area of the triangle they form is zero
/// - The area is calculated using the cross product of two vectors
///
/// # Arguments
/// * `vertices` - A slice of references to vertices to validate
/// * `precision` - The maximum allowed area magnitude to consider vertices collinear (in square meters)
///
/// # Returns
/// * `true` if the vertices are collinear within the specified precision, `false` otherwise
///
/// # Examples
/// ```
/// use harmony_arch::domain::{Vertex, Point, validate_collinear_vertices};
///
/// let v1 = Vertex { id: uuid::Uuid::new_v4(), position: Point { x: 0.0, y: 0.0, z: 0.0 } };
/// let v2 = Vertex { id: uuid::Uuid::new_v4(), position: Point { x: 1.0, y: 0.0, z: 0.0 } };
/// let v3 = Vertex { id: uuid::new_v4(), position: Point { x: 2.0, y: 0.0, z: 0.0 } };
///
/// assert!(validate_collinear_vertices(&[&v1, &v2, &v3], 0.001));
/// ```
use crate::domain::{Vector, Vertex};

/// Validate that the vertices are colinear
pub fn validate_colinear_vertices(vertices: &[&Vertex], precision: f32) -> bool {
    // For 2 or fewer vertices, they are always collinear
    if vertices.len() <= 2 {
        return true;
    }

    // We need at least 3 vertices to form a line
    if vertices.len() < 3 {
        return true;
    }

    // Use the first two vertices to define the line direction
    let v1 = &vertices[0].position;
    let v2 = &vertices[1].position;

    // Calculate the line direction vector
    let line_direction = Vector {
        x: v2.x - v1.x,
        y: v2.y - v1.y,
        z: v2.z - v1.z,
    };

    // Check if the line direction is zero (degenerate case)
    // Use squared magnitude to avoid square root
    let line_magnitude_squared = line_direction.x * line_direction.x
        + line_direction.y * line_direction.y
        + line_direction.z * line_direction.z;

    // Pre-calculate precision squared to avoid repeated multiplication
    let precision_squared = precision * precision;

    if line_magnitude_squared <= precision_squared {
        // If the first two points are essentially the same, all points must be the same
        // Check that all vertices are within precision of the first vertex
        // Use squared distance to avoid square root
        for vertex in vertices.iter().skip(1) {
            let dx = vertex.position.x - v1.x;
            let dy = vertex.position.y - v1.y;
            let dz = vertex.position.z - v1.z;
            let distance_squared = dx * dx + dy * dy + dz * dz;
            if distance_squared > precision_squared {
                return false;
            }
        }
        return true;
    }

    // For each remaining vertex, check if it lies on the line defined by v1 and v2
    // Use squared cross product magnitude to avoid square root
    for vertex in vertices.iter().skip(2) {
        // Calculate vector from v1 to current vertex
        let to_vertex = Vector {
            x: vertex.position.x - v1.x,
            y: vertex.position.y - v1.y,
            z: vertex.position.z - v1.z,
        };

        // Calculate cross product between line direction and vector to vertex
        // If the vertex lies on the line, the cross product should be zero
        let cross = cross_product(&line_direction, &to_vertex);

        // Use squared magnitude to avoid expensive square root
        let cross_magnitude_squared = cross.x * cross.x + cross.y * cross.y + cross.z * cross.z;

        // If cross product magnitude is greater than precision, vertex is not on the line
        if cross_magnitude_squared > precision_squared {
            return false;
        }
    }

    true
}

/// Calculate the cross product of two vectors
fn cross_product(a: &Vector, b: &Vector) -> Vector {
    Vector {
        x: a.y * b.z - a.z * b.y,
        y: a.z * b.x - a.x * b.z,
        z: a.x * b.y - a.y * b.x,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::primitives::Vertex;
    use crate::domain::Point;
    use uuid::Uuid;

    fn create_vertex(x: f32, y: f32, z: f32) -> Vertex {
        Vertex {
            id: Uuid::new_v4(),
            position: Point { x, y, z },
        }
    }

    #[test]
    fn test_edge_cases() {
        // Empty and single vertex cases
        let empty: &[&Vertex] = &[];
        assert!(validate_colinear_vertices(empty, 0.0));

        let v1 = create_vertex(0.0, 0.0, 0.0);
        let single = &[&v1];
        assert!(validate_colinear_vertices(single, 0.0));

        // Two vertices are always collinear
        let v2 = create_vertex(1.0, 0.0, 0.0);
        let two = &[&v1, &v2];
        assert!(validate_colinear_vertices(two, 0.0));
    }

    #[test]
    fn test_basic_collinearity() {
        // Three collinear points
        let v1 = create_vertex(0.0, 0.0, 0.0);
        let v2 = create_vertex(1.0, 0.0, 0.0);
        let v3 = create_vertex(2.0, 0.0, 0.0);
        let vertices = &[&v1, &v2, &v3];
        assert!(validate_colinear_vertices(vertices, 0.0));

        // Three non-collinear points
        let v4 = create_vertex(0.0, 1.0, 0.0);
        let vertices_non = &[&v1, &v2, &v4];
        assert!(!validate_colinear_vertices(vertices_non, 0.1));
    }

    #[test]
    fn test_precision_scaling() {
        let v1 = create_vertex(0.0, 0.0, 0.0);
        let v2 = create_vertex(1.0, 0.0, 0.0);
        let v3 = create_vertex(0.5, 0.001, 0.0); // 1mm deviation

        let vertices = &[&v1, &v2, &v3];

        // Test precision levels from exact to schematic
        assert!(!validate_colinear_vertices(vertices, 0.0)); // Exact - fails
        assert!(!validate_colinear_vertices(vertices, 0.0001)); // 0.1mm - fails
        assert!(validate_colinear_vertices(vertices, 0.001)); // 1mm - passes
        assert!(validate_colinear_vertices(vertices, 0.01)); // 1cm - passes
        assert!(validate_colinear_vertices(vertices, 0.1)); // 10cm - passes
    }

    #[test]
    fn test_architectural_scale_examples() {
        // 10m building line with 1cm deviation
        let v1 = create_vertex(0.0, 0.0, 0.0);
        let v2 = create_vertex(10.0, 0.0, 0.0);
        let v3 = create_vertex(5.0, 0.01, 0.0);

        let vertices = &[&v1, &v2, &v3];

        // Test realistic architectural tolerances
        assert!(!validate_colinear_vertices(vertices, 0.001)); // 1mm - fails
        assert!(!validate_colinear_vertices(vertices, 0.01)); // 1cm - fails
        assert!(validate_colinear_vertices(vertices, 0.1)); // 10cm - passes
        assert!(validate_colinear_vertices(vertices, 1.0)); // 1m - passes
    }

    #[test]
    fn test_four_vertices_validation() {
        let v1 = create_vertex(0.0, 0.0, 0.0);
        let v2 = create_vertex(1.0, 0.0, 0.0);
        let v3 = create_vertex(2.0, 0.0, 0.0);
        let v4 = create_vertex(1.0, 1.0, 0.0);

        let vertices = &[&v1, &v2, &v3, &v4];

        // First three are collinear, but fourth is not
        // Function should now check ALL vertices and return false
        assert!(!validate_colinear_vertices(vertices, 0.1));

        // All four vertices should be collinear
        let v4_collinear = create_vertex(3.0, 0.0, 0.0);
        let vertices_collinear = &[&v1, &v2, &v3, &v4_collinear];
        assert!(validate_colinear_vertices(vertices_collinear, 0.1));
    }

    #[test]
    fn test_all_vertices_validation() {
        let v1 = create_vertex(0.0, 0.0, 0.0);
        let v2 = create_vertex(1.0, 0.0, 0.0);
        let v3 = create_vertex(2.0, 0.0, 0.0);
        let v4 = create_vertex(3.0, 0.0, 0.0);
        let v5 = create_vertex(4.0, 0.0, 0.0);

        // All vertices are collinear
        let vertices_collinear = &[&v1, &v2, &v3, &v4, &v5];
        assert!(validate_colinear_vertices(vertices_collinear, 0.1));

        // First 4 are collinear, but 5th is not
        let v5_off_line = create_vertex(4.0, 0.1, 0.0);
        let vertices_mixed = &[&v1, &v2, &v3, &v4, &v5_off_line];
        assert!(!validate_colinear_vertices(vertices_mixed, 0.05)); // Use 5cm precision
    }
}
