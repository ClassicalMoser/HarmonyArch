/// Distinct validation functions
/// These functions iterate over a collection of primitives and check if they have distinct ids
/// If they do, the function returns true, otherwise it returns false
/// This is used to prevent degenerate geometries
use std::collections::HashSet;
use uuid::Uuid;

/// Generic validation function for distinct IDs
pub fn validate_distinct_ids(items: &[&Uuid]) -> bool {
    let mut distinct_ids = HashSet::new();

    for item in items {
        if !distinct_ids.insert(**item) {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    fn create_distinct_ids(count: usize) -> Vec<Uuid> {
        let mut vertices = Vec::new();
        for _ in 0..count {
            let id = Uuid::new_v4();
            vertices.push(id);
        }
        vertices
    }

    fn create_overlapping_ids(count: usize) -> Vec<Uuid> {
        let mut vertices = Vec::new();
        for _ in 0..count - 1 {
            let id = Uuid::new_v4();
            vertices.push(id);
        }
        let overlapping_id = vertices[0].clone();
        vertices.push(overlapping_id);
        vertices
    }

    #[test]
    fn test_generic_validate_distinct_ids() {
        let items = create_distinct_ids(5);
        assert!(validate_distinct_ids(&items.iter().collect::<Vec<&Uuid>>()));
    }

    #[test]
    fn test_generic_validate_distinct_ids_with_duplicates() {
        let items = create_overlapping_ids(5);
        assert!(!validate_distinct_ids(
            &items.iter().collect::<Vec<&Uuid>>()
        ));
    }
}
