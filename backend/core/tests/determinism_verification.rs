use rts_core::{Node, Edge, step_sparse_parallel};
use rayon::ThreadPoolBuilder;

fn run_simulation(num_threads: usize, nodes: &[Node], edges: &[Edge]) -> Vec<f32> {
    let pool = ThreadPoolBuilder::new().num_threads(num_threads).build().unwrap();
    let mut current = nodes.to_vec();
    let mut next = nodes.to_vec();
    let n = nodes.len();
    
    pool.install(|| {
        step_sparse_parallel(&current, &mut next, edges, n);
    });
    
    next.iter().map(|n| n.theta).collect()
}

#[test]
fn test_parallel_determinism() {
    let n = 100;
    let mut nodes = vec![Node::default(); n];
    for i in 0..n {
        nodes[i].theta = i as f32 * 0.1;
        nodes[i].e = 1.0;
        nodes[i].ec = 0.5;
    }
    
    let mut edges = Vec::new();
    for i in 0..n {
        edges.push(Edge { src: ((i + 1) % n) as u32, dst: i as u32, weight: 0.5 });
        edges.push(Edge { src: ((i + 2) % n) as u32, dst: i as u32, weight: 0.2 });
    }
    // CRITICAL: Sort edges to ensure deterministic gather
    edges.sort_by_key(|e| (e.dst, e.src));

    let res1 = run_simulation(1, &nodes, &edges);
    let res4 = run_simulation(4, &nodes, &edges);
    let res16 = run_simulation(16, &nodes, &edges);

    for i in 0..n {
        assert_eq!(res1[i].to_bits(), res4[i].to_bits(), "Mismatch at index {} between 1 and 4 threads", i);
        assert_eq!(res1[i].to_bits(), res16[i].to_bits(), "Mismatch at index {} between 1 and 16 threads", i);
    }
}
