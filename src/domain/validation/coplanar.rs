/// Coplanar validation functions
/// These functions iterate over a collection of primitives and check if they are coplanar
/// If they are, the function returns true, otherwise it returns false
/// This is used to prevent degenerate geometries
use crate::domain::{Vector, Vertex};

/// Validate that vertices are coplanar
///
/// This function checks if all vertices in the given collection lie on the same plane.
///
/// # Mathematical Approach
/// - For 3 or fewer vertices, they are always coplanar
/// - For 4+ vertices, we use the first 3 vertices to define a plane
/// - We then check if all other vertices lie on that plane using the point-plane distance formula
///
/// # Arguments
/// * `vertices` - A vector of references to vertices to validate
/// * `tolerance` - The maximum allowed deviation from the plane in meters
///
/// # Important Note on Tolerance Scaling
/// The tolerance parameter is in meters and represents the actual geometric distance
/// from a vertex to the plane. The function automatically normalizes the distance
/// calculation, so you can use the same tolerance values regardless of model scale:
/// - Small models (furniture): 0.001m (1mm) to 0.01m (1cm)
/// - Medium models (rooms): 0.01m (1cm) to 0.1m (10cm)  
/// - Large models (buildings): 0.1m (10cm) to 1.0m (1m)
///
/// # Returns
/// * `true` if all vertices are coplanar within the specified tolerance, `false` otherwise
///
/// # Examples
/// ```
/// use harmony_arch::domain::{Vertex, Point, validate_coplanar_vertices};
///
/// let v1 = Vertex { id: uuid::Uuid::new_v4(), position: Point { x: 0.0, y: 0.0, z: 0.0 } };
/// let v2 = Vertex { id: uuid::Uuid::new_v4(), position: Point { x: 1.0, y: 0.0, z: 0.0 } };
/// let v3 = Vertex { id: uuid::Uuid::new_v4(), position: Point { x: 0.0, y: 1.0, z: 0.0 } };
/// let v4 = Vertex { id: uuid::Uuid::new_v4(), position: Point { x: 1.0, y: 1.0, z: 0.0 } };
///
/// assert!(validate_coplanar_vertices(vec![&v1, &v2, &v3, &v4], 0.001));
/// ```
pub fn validate_coplanar_vertices(vertices: &[&Vertex], tolerance: f32) -> bool {
    // For 3 or fewer vertices, they are always coplanar
    if vertices.len() <= 3 {
        return true;
    }

    // We need at least 3 vertices to define a plane
    if vertices.len() < 3 {
        return true;
    }

    // Get the first 3 vertices to define the plane
    let v1 = &vertices[0].position;
    let v2 = &vertices[1].position;
    let v3 = &vertices[2].position;

    // Calculate two vectors in the plane
    let vec1 = Vector {
        x: v2.x - v1.x,
        y: v2.y - v1.y,
        z: v2.z - v1.z,
    };

    let vec2 = Vector {
        x: v3.x - v1.x,
        y: v3.y - v1.y,
        z: v3.z - v1.z,
    };

    // Calculate the normal vector of the plane using cross product
    let normal = cross_product(&vec1, &vec2);

    // If the normal vector is zero (degenerate case), all points are collinear
    if normal.x == 0.0 && normal.y == 0.0 && normal.z == 0.0 {
        return true;
    }

    // Calculate the plane equation: ax + by + cz + d = 0
    // where (a, b, c) is the normal vector and d = -(ax₀ + by₀ + cz₀)
    let d = -(normal.x * v1.x + normal.y * v1.y + normal.z * v1.z);

    // Check if all other vertices lie on the plane
    // A point (x, y, z) lies on the plane if ax + by + cz + d = 0
    // We use the provided tolerance for floating-point precision
    for vertex in vertices.iter().skip(3) {
        let raw_distance = normal.x * vertex.position.x
            + normal.y * vertex.position.y
            + normal.z * vertex.position.z
            + d;

        // Normalize by the magnitude of the normal vector to get actual geometric distance
        let normal_magnitude =
            (normal.x * normal.x + normal.y * normal.y + normal.z * normal.z).sqrt();
        let actual_distance = raw_distance.abs() / normal_magnitude;

        if actual_distance > tolerance {
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
        assert!(validate_coplanar_vertices(empty, 0.0));

        let v1 = create_vertex(0.0, 0.0, 0.0);
        let single = &[&v1];
        assert!(validate_coplanar_vertices(single, 0.0));

        // Two vertices are always coplanar
        let v2 = create_vertex(1.0, 0.0, 0.0);
        let two = &[&v1, &v2];
        assert!(validate_coplanar_vertices(two, 0.0));

        // Three vertices are always coplanar
        let v3 = create_vertex(0.0, 1.0, 0.0);
        let three = &[&v1, &v2, &v3];
        assert!(validate_coplanar_vertices(three, 0.0));
    }

    #[test]
    fn test_basic_coplanarity() {
        // Four coplanar points
        let v1 = create_vertex(0.0, 0.0, 0.0);
        let v2 = create_vertex(1.0, 0.0, 0.0);
        let v3 = create_vertex(0.0, 1.0, 0.0);
        let v4 = create_vertex(1.0, 1.0, 0.0);
        let vertices = &[&v1, &v2, &v3, &v4];
        assert!(validate_coplanar_vertices(vertices, 0.001));

        // Four non-coplanar points
        let v5 = create_vertex(0.5, 0.5, 0.5); // This vertex is not on the plane
        let vertices_non = &[&v1, &v2, &v3, &v5];
        assert!(!validate_coplanar_vertices(vertices_non, 0.001));
    }

    #[test]
    fn test_degenerate_cases() {
        // Collinear points should still be considered coplanar
        let v1 = create_vertex(0.0, 0.0, 0.0);
        let v2 = create_vertex(1.0, 0.0, 0.0);
        let v3 = create_vertex(2.0, 0.0, 0.0);
        let v4 = create_vertex(3.0, 0.0, 0.0);
        let vertices = &[&v1, &v2, &v3, &v4];
        assert!(validate_coplanar_vertices(vertices, 0.001));
    }

    #[test]
    fn test_precision_scaling() {
        let v1 = create_vertex(0.0, 0.0, 0.0);
        let v2 = create_vertex(1.0, 0.0, 0.0);
        let v3 = create_vertex(0.0, 1.0, 0.0);
        let v4 = create_vertex(1.0, 1.0, 0.01); // 1cm deviation

        let vertices = &[&v1, &v2, &v3, &v4];

        // Test precision levels from exact to schematic
        assert!(!validate_coplanar_vertices(vertices, 0.001)); // 1mm - fails
        assert!(validate_coplanar_vertices(vertices, 0.01)); // 1cm - passes
        assert!(validate_coplanar_vertices(vertices, 0.1)); // 10cm - passes
    }

    #[test]
    fn test_architectural_scale_examples() {
        // 10m building with 5cm deviation
        let v1 = create_vertex(0.0, 0.0, 0.0);
        let v2 = create_vertex(10.0, 0.0, 0.0);
        let v3 = create_vertex(0.0, 10.0, 0.0);
        let v4 = create_vertex(10.0, 10.0, 0.05);

        let vertices = &[&v1, &v2, &v3, &v4];

        // Test realistic architectural tolerances
        assert!(!validate_coplanar_vertices(vertices, 0.01)); // 1cm - fails
        assert!(validate_coplanar_vertices(vertices, 0.1)); // 10cm - passes
        assert!(validate_coplanar_vertices(vertices, 1.0)); // 1m - passes
    }

    #[test]
    fn test_furniture_scale_examples() {
        // 1m furniture with 1mm deviation
        let v1 = create_vertex(0.0, 0.0, 0.0);
        let v2 = create_vertex(1.0, 0.0, 0.0);
        let v3 = create_vertex(0.0, 1.0, 0.0);
        let v4 = create_vertex(1.0, 1.0, 0.001);

        let vertices = &[&v1, &v2, &v3, &v4];

        // Test furniture-scale tolerances
        assert!(!validate_coplanar_vertices(vertices, 0.0001)); // 0.1mm - fails
        assert!(validate_coplanar_vertices(vertices, 0.001)); // 1mm - passes
        assert!(validate_coplanar_vertices(vertices, 0.01)); // 1cm - passes
    }

    #[test]
    fn test_zero_tolerance_exact_validation() {
        let v1 = create_vertex(0.0, 0.0, 0.0);
        let v2 = create_vertex(1.0, 0.0, 0.0);
        let v3 = create_vertex(0.0, 1.0, 0.0);
        let v4 = create_vertex(1.0, 1.0, 0.0);

        let vertices = &[&v1, &v2, &v3, &v4];

        // Zero tolerance - exact validation
        assert!(validate_coplanar_vertices(vertices, 0.0));

        // Add tiny deviation
        let v5 = create_vertex(0.5, 0.5, 0.0000001);
        let vertices_with_deviation = &[&v1, &v2, &v3, &v4, &v5];

        // Should fail with zero tolerance
        assert!(!validate_coplanar_vertices(vertices_with_deviation, 0.0));
    }

    #[test]
    fn test_integration_with_vertex_registry() {
        use crate::domain::primitives::VertexRegistry;

        // Create a vertex registry and add vertices
        let mut registry = VertexRegistry::create_new();
        let v1_id = registry.create_and_store(Point {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        });
        let v2_id = registry.create_and_store(Point {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        });
        let v3_id = registry.create_and_store(Point {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        });
        let v4_id = registry.create_and_store(Point {
            x: 1.0,
            y: 1.0,
            z: 0.0,
        });
        let v5_id = registry.create_and_store(Point {
            x: 0.5,
            y: 0.5,
            z: 1.0,
        });

        // Get references to the vertices
        let v1 = registry.get(&v1_id).unwrap();
        let v2 = registry.get(&v2_id).unwrap();
        let v3 = registry.get(&v3_id).unwrap();
        let v4 = registry.get(&v4_id).unwrap();
        let v5 = registry.get(&v5_id).unwrap();

        // Test coplanar validation
        let vertices = &[v1, v2, v3, v4];
        assert!(validate_coplanar_vertices(vertices, 0.001));

        // Test with non-coplanar vertex
        let vertices_with_v5 = &[v1, v2, v3, v4, v5];
        assert!(!validate_coplanar_vertices(vertices_with_v5, 0.001));
    }
}
