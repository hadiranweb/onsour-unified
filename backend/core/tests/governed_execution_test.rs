use core_engine::{Node, Edge, ThermodynamicGovernor, SystemMetrics, LogicalTimestamp};
use core_engine::graph::step_governed_buffered;

#[test]
fn test_governed_step_accept_and_rollback() {
    let mut governor = ThermodynamicGovernor::new(0.01, 0.005, 0.25, 0.3).unwrap();
    let tick = LogicalTimestamp::new(0, 100);
    let metrics = SystemMetrics::new(0.2, 0.2, 10.0, tick);

    let current_nodes = vec![
        Node { theta: 0.1, e: 1.0, ec: 0.5, _padding: 0 },
        Node { theta: 0.1, e: 1.0, ec: 0.5, _padding: 0 },
    ];
    let mut next_nodes = current_nodes.clone();
    let edges = vec![
        Edge { src: 0, dst: 1, weight: 0.5, _padding: 0 },
        Edge { src: 1, dst: 0, weight: 0.5, _padding: 0 },
    ];

    // 1. Stable transition
    let snapshot = step_governed_buffered(
        &current_nodes,
        &mut next_nodes,
        &edges,
        &mut governor,
        &metrics,
        tick,
        1,
    );
    assert!(snapshot.accepted);
    assert_eq!(snapshot.epoch, 1);

    // 2. Force a violation
    let mut chaotic_next = current_nodes.clone();
    chaotic_next[0].theta = -0.9;
    chaotic_next[1].theta = 0.9;
    
    let snapshot_rollback = governor.apply_governance(
        &current_nodes,
        &mut chaotic_next,
        &metrics,
        tick,
        2
    );
    
    assert!(!snapshot_rollback.accepted, "High dispersion must be rejected");
    assert_eq!(chaotic_next[0].theta, current_nodes[0].theta);
}
