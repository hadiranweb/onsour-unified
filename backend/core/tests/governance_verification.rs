use core_engine::{Node, ThermodynamicGovernor, SystemMetrics, LogicalTimestamp};

#[test]
fn test_entropy_filter_and_rollback() {
    let mut governor = ThermodynamicGovernor::new(0.01, 0.005, 0.25, 0.3).unwrap(); 
    let tick = LogicalTimestamp::new(0, 100);
    let metrics = SystemMetrics::new(0.2, 0.2, 10.0, tick);

    let current = vec![
        Node { theta: 0.1, e: 1.0, ec: 0.5, _padding: 0 },
        Node { theta: 0.1, e: 1.0, ec: 0.5, _padding: 0 },
        Node { theta: 0.1, e: 1.0, ec: 0.5, _padding: 0 },
        Node { theta: 0.1, e: 1.0, ec: 0.5, _padding: 0 },
    ];

    let mut next_valid = vec![
        Node { theta: 0.11, e: 1.0, ec: 0.5, _padding: 0 },
        Node { theta: 0.11, e: 1.0, ec: 0.5, _padding: 0 },
        Node { theta: 0.11, e: 1.0, ec: 0.5, _padding: 0 },
        Node { theta: 0.11, e: 1.0, ec: 0.5, _padding: 0 },
    ];

    let snapshot_valid = governor.apply_governance(&current, &mut next_valid, &metrics, tick, 1);
    assert!(snapshot_valid.accepted);

    let mut next_chaotic = vec![
        Node { theta: -0.9, e: 1.0, ec: 0.5, _padding: 0 },
        Node { theta: -0.3, e: 1.0, ec: 0.5, _padding: 0 },
        Node { theta: 0.3, e: 1.0, ec: 0.5, _padding: 0 },
        Node { theta: 0.9, e: 1.0, ec: 0.5, _padding: 0 },
    ];

    let snapshot_chaotic = governor.apply_governance(&current, &mut next_chaotic, &metrics, tick, 2);
    assert!(!snapshot_chaotic.accepted, "Chaotic transition must be rejected");
}
