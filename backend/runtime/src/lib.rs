use core_engine::{Node, Edge, step_sparse_parallel};
use rayon::prelude::*;

/// A parallel execution domain (Island) for processing graph-based stochastic networks.
pub struct Island {
    pub current_state: Vec<Node>,
    pub next_state: Vec<Node>,
    pub edges: Vec<Edge>,
}

impl Island {
    pub fn new(num_nodes: usize, edges: Vec<Edge>) -> Self {
        let current_state = vec![Node::default(); num_nodes];
        let next_state = vec![Node::default(); num_nodes];
        Self { current_state, next_state, edges }
    }

    /// Executes one epoch using the Two-Phase Gather/Apply mechanism.
    pub fn step(&mut self) {
        let n = self.current_state.len();
        step_sparse_parallel(&self.current_state, &mut self.next_state, &self.edges, n);
        
        // Double Buffering: Swap states for the next epoch
        std::mem::swap(&mut self.current_state, &mut self.next_state);
    }
}
