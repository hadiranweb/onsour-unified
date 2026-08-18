use rts_core::{Node, Edge, step_sparse, step_sparse_buffered, alpha};
use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
struct TestScenario {
    name: String,
    nodes: Vec<NodeData>,
    edges: Vec<EdgeData>,
    steps: usize,
    expected_trajectory: Vec<TrajectoryStep>,
}

#[derive(Deserialize)]
struct NodeData {
    theta: f32,
    e: f32,
    ec: f32,
}

#[derive(Deserialize)]
struct EdgeData {
    src: u32,
    dst: u32,
    weight: f32,
}

#[derive(Deserialize)]
struct TrajectoryStep {
    step: usize,
    thetas: Vec<f32>,
    alphas: Vec<f32>,
}

#[test]
fn test_numeric_equivalence_v04() {
    let content = fs::read_to_string("../../../theory/python_reference/test_vectors.json")
        .expect("Could not read test_vectors.json");
    let scenarios: Vec<TestScenario> = serde_json::from_str(&content).unwrap();

    for scenario in scenarios {
        println!("Testing scenario: {}", scenario.name);
        let mut nodes: Vec<Node> = scenario.nodes.iter().map(|n| Node {
            theta: n.theta,
            e: n.e,
            ec: n.ec,
            _padding: 0,
        }).collect();
        
        let edges: Vec<Edge> = scenario.edges.iter().map(|e| Edge {
            src: e.src,
            dst: e.dst,
            weight: e.weight,
            _padding: 0,
        }).collect();

        let threshold = if scenario.name == "long_trajectory" { 1e-4 } else { 1e-5 };

        let mut expected_idx = 0;
        for step_idx in 0..scenario.steps {
            step_sparse(&mut nodes, &edges);
            
            if expected_idx < scenario.expected_trajectory.len() && scenario.expected_trajectory[expected_idx].step == step_idx {
                let expected = &scenario.expected_trajectory[expected_idx];
                for i in 0..nodes.len() {
                    let diff_theta = (nodes[i].theta - expected.thetas[i]).abs();
                    let _diff_alpha = (alpha(nodes[i].theta) - expected.alphas[i]).abs();
                    
                    assert!(diff_theta <= threshold, 
                        "Scenario {} step {} node {}: theta diff {} > threshold {}", 
                        scenario.name, step_idx, i, diff_theta, threshold);
                }
                expected_idx += 1;
            }
        }
        println!("Scenario {} PASSED.", scenario.name);
    }
}

#[test]
fn test_parallel_determinism() {
    let n = 1000;
    let current_nodes = vec![Node { theta: 0.1, e: 10.0, ec: 5.0, _padding: 0 }; n];
    let mut edges = Vec::new();
    for i in 0..n {
        edges.push(Edge { src: i as u32, dst: ((i + 1) % n) as u32, weight: 0.5, _padding: 0 });
        edges.push(Edge { src: i as u32, dst: ((i + 2) % n) as u32, weight: 0.2, _padding: 0 });
    }

    // Run 1
    let mut next_1 = vec![Node::default(); n];
    step_sparse_buffered(&current_nodes, &mut next_1, &edges);

    // Run 2
    let mut next_2 = vec![Node::default(); n];
    step_sparse_buffered(&current_nodes, &mut next_2, &edges);

    for i in 0..n {
        assert_eq!(next_1[i].theta.to_bits(), next_2[i].theta.to_bits(), 
            "Non-deterministic result at node {}! Bit-exact match failed.", i);
    }
    println!("Parallel determinism verified.");
}
