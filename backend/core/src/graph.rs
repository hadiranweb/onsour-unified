use wasm_bindgen::prelude::*;
use crate::state::{Node, Edge};
use crate::math::step_node_math;
use rayon::prelude::*;

/// Executes a sparse graph update using a Double Buffering model for correctness.
/// 
/// CORRECTNESS INVARIANTS:
/// 1. Immutable Source: All reads for epoch t originate from `current_nodes` [State(t)].
/// 2. Independent Write: Each element in `next_nodes` [State(t+1)] is written by exactly one worker.
/// 3. Zero Race Conditions: Phase separation ensures no concurrent conflicting writes.
/// 4. Bit-Exact Determinism: Neighbor sums are computed in a fixed order (sorted by source).
pub fn step_sparse_buffered(current_nodes: &[Node], next_nodes: &mut [Node], edges: &[Edge]) {
    let n = current_nodes.len();
    assert_eq!(n, next_nodes.len(), "Buffer size mismatch");
    
    // Phase 1: Gather (Deterministic Parallel Reduction)
    // To ensure bit-exact results across thread counts, we must:
    // a) Group edges by destination (so each neighbor_sum[i] has ONE writer).
    // b) Sort incoming edges for each node by source index to fix summation order.
    
    // NOTE: In production, the adjacency list should be pre-built and sorted.
    // For this implementation, we build it on the fly for correctness-first.
    let mut adjacency: Vec<Vec<(usize, f32)>> = vec![vec![]; n];
    for edge in edges {
        let src = edge.src as usize;
        let dst = edge.dst as usize;
        if src < n && dst < n {
            adjacency[dst].push((src, edge.weight));
        }
    }

    // Parallelize over destination nodes - each worker owns neighbor_sums[i]
    let neighbor_sums: Vec<f32> = (0..n)
        .into_par_iter()
        .map(|i| {
            let mut node_edges = adjacency[i].clone();
            // Fix order: Sort by source index
            node_edges.sort_by_key(|&(src, _)| src);
            
            // Deterministic Fold (Sequential sum within the worker)
            node_edges.iter().fold(0.0, |acc, &(src, w)| {
                acc + (current_nodes[src].theta * w)
            })
        })
        .collect();
    
    // Phase 2: Apply (Embarrassingly Parallel)
    next_nodes.par_iter_mut().enumerate().for_each(|(i, node)| {
        node.theta = step_node_math(
            current_nodes[i].theta, 
            current_nodes[i].e, 
            current_nodes[i].ec, 
            neighbor_sums[i]
        );
        // Field preservation
        node.e = current_nodes[i].e;
        node.ec = current_nodes[i].ec;
        node._padding = current_nodes[i]._padding;
    });
}

/// Single-buffer API wrapper.
pub fn step_sparse_impl(nodes: &mut [Node], edges: &[Edge]) {
    let mut next_nodes = nodes.to_vec();
    step_sparse_buffered(nodes, &mut next_nodes, edges);
    nodes.copy_from_slice(&next_nodes);
}

#[wasm_bindgen]
pub fn step_sparse_js(nodes: JsValue, edges: JsValue) -> JsValue {
    let mut nodes_vec: Vec<Node> = serde_wasm_bindgen::from_value(nodes).unwrap();
    let edges_vec: Vec<Edge> = serde_wasm_bindgen::from_value(edges).unwrap();
    
    step_sparse_impl(&mut nodes_vec, &edges_vec);
    
    serde_wasm_bindgen::to_value(&nodes_vec).unwrap()
}
