use core_engine::{Node, ThermodynamicGovernor, SystemMetrics};

#[test]
fn test_governed_step_accept_and_rollback() {
    let mut governor = ThermodynamicGovernor::new_with_bounds(0.01, 0.005, 0.25, 0.3).unwrap();
    let metrics = SystemMetrics::new(0.2, 0.2, 10.0, 100);

    // Uniform state (low entropy)
    let current_nodes = vec![
        Node { theta: 0.1, e: 1.0, ec: 0.5, _padding: 0 },
        Node { theta: 0.1, e: 1.0, ec: 0.5, _padding: 0 },
        Node { theta: 0.1, e: 1.0, ec: 0.5, _padding: 0 },
        Node { theta: 0.1, e: 1.0, ec: 0.5, _padding: 0 },
    ];

    let next_nodes_valid = vec![
        Node { theta: 0.11, e: 1.0, ec: 0.5, _padding: 0 },
        Node { theta: 0.11, e: 1.0, ec: 0.5, _padding: 0 },
        Node { theta: 0.11, e: 1.0, ec: 0.5, _padding: 0 },
        Node { theta: 0.11, e: 1.0, ec: 0.5, _padding: 0 },
    ];

    let res_accept = governor.validate_transition_with_snapshot(
        &current_nodes,
        &next_nodes_valid,
        &metrics,
        100,
        1,
    );
    assert!(res_accept.is_ok(), "Stable transition must be accepted");

    // Dispersed state (high entropy spike)
    let next_nodes_chaotic = vec![
        Node { theta: -0.9, e: 1.0, ec: 0.5, _padding: 0 },
        Node { theta: -0.3, e: 1.0, ec: 0.5, _padding: 0 },
        Node { theta: 0.3, e: 1.0, ec: 0.5, _padding: 0 },
        Node { theta: 0.9, e: 1.0, ec: 0.5, _padding: 0 },
    ];

    let res_rollback = governor.validate_transition_with_snapshot(
        &current_nodes,
        &next_nodes_chaotic,
        &metrics,
        100,
        2,
    );
    assert!(res_rollback.is_err(), "Chaotic transition must trigger rollback");
    println!("Successfully verified entropy violation rejection: {}", res_rollback.unwrap_err());
}
