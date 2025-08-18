/// Geometric constraints for the domain layer
/// Constraints define relationships between geometric primitives
/// They are used to maintain geometric integrity and enforce design rules
use crate::domain::{Point, Vector};
use std::any::Any;
use std::fmt::Debug;

/// Individual constraint modules
pub mod distance;
/// Horizontal constraints for ground-parallel geometry
pub mod horizontal;
/// Vertical constraints for ground-perpendicular geometry
pub mod vertical;

/// Re-export constraint types for convenience
pub use distance::DistanceConstraint;
pub use horizontal::{HeightConstraint, HorizontalConstraint};
pub use vertical::{VerticalAlignmentConstraint, VerticalConstraint};

/// A geometric constraint that can be solved
/// This trait provides a common interface for all constraint types
pub trait Constraint: Debug + Send + Sync + Any {
    /// Get the constraint type identifier
    fn constraint_type(&self) -> &'static str;

    /// Calculate the residual (how far from satisfied this constraint is)
    /// Returns 0.0 when fully satisfied, positive values indicate violation
    fn residual(&self, geometry: &GeometryState) -> f32;

    /// Get the Jacobian row for this constraint
    /// This defines how changes in geometry affect the constraint
    fn jacobian_row(&self, geometry: &GeometryState) -> JacobianRow;

    /// Get the constraint's priority (lower numbers = higher priority)
    fn priority(&self) -> u32;

    /// Get a reference to Any for downcasting
    fn as_any(&self) -> &dyn Any;
}

/// A row in the Jacobian matrix representing one constraint
#[derive(Debug, Clone)]
pub struct JacobianRow {
    /// The constraint index
    pub constraint_idx: usize,
    /// The geometry variable indices and their partial derivatives
    pub derivatives: Vec<(usize, f32)>,
}

/// The current state of all geometry in the system
#[derive(Debug, Clone)]
pub struct GeometryState {
    /// All points in the system
    pub points: Vec<Point>,
    /// All vectors in the system  
    pub vectors: Vec<Vector>,
}

impl GeometryState {
    /// Create a new geometry state
    pub fn new() -> Self {
        Self {
            points: Vec::new(),
            vectors: Vec::new(),
        }
    }

    /// Add a point to the state
    pub fn add_point(&mut self, point: Point) -> usize {
        let idx = self.points.len();
        self.points.push(point);
        idx
    }

    /// Get a point by index
    pub fn get_point(&self, idx: usize) -> Option<&Point> {
        self.points.get(idx)
    }

    /// Get a mutable reference to a point
    pub fn get_point_mut(&mut self, idx: usize) -> Option<&mut Point> {
        self.points.get_mut(idx)
    }

    /// Get the total number of geometry variables
    pub fn variable_count(&self) -> usize {
        self.points.len() * 3 + self.vectors.len() * 3
    }
}

/// Result of constraint solving
#[derive(Debug, Clone)]
pub enum SolverResult {
    /// Successfully converged in given number of iterations
    Converged(usize),
    /// Maximum iterations reached without convergence
    MaxIterationsReached,
    /// Solver failed (singular matrix, etc.)
    Failed(String),
}

/// The main constraint solver
/// This is the interface that other parts of the system use
pub struct ConstraintSolver {
    /// The current geometry state
    geometry: GeometryState,
    /// All constraints in the system
    constraints: Vec<Box<dyn Constraint>>,
    /// Solver configuration
    config: SolverConfig,
}

/// Configuration for the constraint solver
#[derive(Debug, Clone)]
pub struct SolverConfig {
    /// Maximum number of iterations
    pub max_iterations: usize,
    /// Convergence tolerance
    pub tolerance: f32,
    /// Damping factor for stability (0.0 to 1.0)
    pub damping: f32,
}

impl Default for SolverConfig {
    fn default() -> Self {
        Self {
            max_iterations: 100,
            tolerance: 1e-6,
            damping: 0.5,
        }
    }
}

impl ConstraintSolver {
    /// Create a new constraint solver
    pub fn new() -> Self {
        Self {
            geometry: GeometryState::new(),
            constraints: Vec::new(),
            config: SolverConfig::default(),
        }
    }

    /// Set solver configuration
    pub fn with_config(mut self, config: SolverConfig) -> Self {
        self.config = config;
        self
    }

    /// Add a constraint to the system
    pub fn add_constraint(&mut self, constraint: Box<dyn Constraint>) {
        self.constraints.push(constraint);
    }

    /// Add a point to the geometry state
    pub fn add_point(&mut self, point: Point) -> usize {
        self.geometry.add_point(point)
    }

    /// Get the current geometry state
    pub fn geometry(&self) -> &GeometryState {
        &self.geometry
    }

    /// Get mutable access to geometry state
    pub fn geometry_mut(&mut self) -> &mut GeometryState {
        &mut self.geometry
    }

    /// Solve all constraints in the system
    pub fn solve(&mut self) -> SolverResult {
        if self.constraints.is_empty() {
            return SolverResult::Converged(0);
        }

        // Sort constraints by priority
        self.constraints.sort_by_key(|c| c.priority());

        // Simple iterative solver for now
        // TODO: Replace with proper Jacobian-based solver
        self.solve_iterative()
    }

    /// Simple iterative solver (placeholder for proper solver)
    fn solve_iterative(&mut self) -> SolverResult {
        for iteration in 0..self.config.max_iterations {
            let mut max_residual: f32 = 0.0;

            // Check all constraints
            for constraint in &self.constraints {
                let residual = constraint.residual(&self.geometry);
                max_residual = max_residual.max(residual);
            }

            // Check convergence
            if max_residual < self.config.tolerance {
                return SolverResult::Converged(iteration + 1);
            }

            // Simple relaxation step
            // TODO: Replace with proper Newton-Raphson iteration
            self.relax_constraints();
        }

        SolverResult::MaxIterationsReached
    }

    /// Simple constraint relaxation (placeholder)
    fn relax_constraints(&mut self) {
        // Simple relaxation: move points to satisfy distance constraints
        let mut distance_constraints = Vec::new();

        // Collect distance constraints first to avoid borrowing issues
        for constraint in &self.constraints {
            if let Some(distance_constraint) =
                constraint.as_any().downcast_ref::<DistanceConstraint>()
            {
                distance_constraints.push(distance_constraint.clone());
            }
        }

        // Now apply the constraints
        for constraint in &distance_constraints {
            self.relax_distance_constraint(constraint);
        }
    }

    /// Relax a distance constraint by moving points
    fn relax_distance_constraint(&mut self, constraint: &DistanceConstraint) {
        let point_a = self.geometry.get_point(constraint.point_a_idx).unwrap();
        let point_b = self.geometry.get_point(constraint.point_b_idx).unwrap();

        let vector = crate::domain::measure_vector(point_a, point_b);
        let current_distance =
            (vector.x * vector.x + vector.y * vector.y + vector.z * vector.z).sqrt();

        if current_distance == 0.0 {
            // Avoid division by zero - move point B slightly
            let point_b_mut = self.geometry.get_point_mut(constraint.point_b_idx).unwrap();
            point_b_mut.x += 0.1;
            return;
        }

        let scale_factor = constraint.required_distance / current_distance;
        let adjustment_x = vector.x * (scale_factor - 1.0) * self.config.damping;
        let adjustment_y = vector.y * (scale_factor - 1.0) * self.config.damping;
        let adjustment_z = vector.z * (scale_factor - 1.0) * self.config.damping;

        // Move both points to satisfy the constraint
        let point_a_mut = self.geometry.get_point_mut(constraint.point_a_idx).unwrap();
        point_a_mut.x -= adjustment_x * 0.5;
        point_a_mut.y -= adjustment_y * 0.5;
        point_a_mut.z -= adjustment_z * 0.5;

        let point_b_mut = self.geometry.get_point_mut(constraint.point_b_idx).unwrap();
        point_b_mut.x += adjustment_x * 0.5;
        point_b_mut.y += adjustment_y * 0.5;
        point_b_mut.z += adjustment_z * 0.5;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constraint_solver_basic() {
        let mut solver = ConstraintSolver::new();

        // Add some geometry
        let point_a_idx = solver.add_point(Point {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        });
        let point_b_idx = solver.add_point(Point {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        });

        // Add a distance constraint
        let constraint = DistanceConstraint {
            point_a_idx,
            point_b_idx,
            required_distance: 2.0,
            priority: 1,
        };

        solver.add_constraint(Box::new(constraint));

        // Solve constraints
        let result = solver.solve();
        match result {
            SolverResult::Converged(iterations) => {
                println!("Converged in {} iterations", iterations);
            }
            _ => panic!("Solver failed to converge"),
        }
    }
}
