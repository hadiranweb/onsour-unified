use crate::state::{Node, Edge};
use crate::math::step_node_math;
use rayon::prelude::*;

/// Deterministic Two-Phase Epoch Execution
pub fn step_sparse_parallel(
    current_nodes: &[Node],
    next_nodes: &mut [Node],
    edges: &[Edge],
    num_nodes: usize,
) {
    // Phase A: Gather (Deterministic Neighbor Sums)
    // To ensure bit-exact determinism across threads:
    // 1. Edges MUST be pre-sorted by (dst, src)
    // 2. We use sequential fold within each node's parallel task
    
    let mut neighbor_sums = vec![0.0; num_nodes];
    
    // In a real implementation, we would pre-build an adjacency list (CSR)
    // for efficiency. Here we simulate the deterministic gather.
    neighbor_sums.par_iter_mut().enumerate().for_each(|(i, sum)| {
        *sum = edges.iter()
            .filter(|e| e.dst as usize == i)
            .fold(0.0, |acc, e| {
                acc + (e.weight * current_nodes[e.src as usize].theta)
            });
    });

    // Phase B: Apply (Independent Writes)
    next_nodes.par_iter_mut().enumerate().for_each(|(i, node)| {
        let current = &current_nodes[i];
        node.theta = step_node_math(current.theta, current.e, current.ec, neighbor_sums[i] as f32);
        node.e = current.e;
        node.ec = current.ec;
    });
}
