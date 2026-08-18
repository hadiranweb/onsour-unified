use core_engine::{Node, ThermodynamicGovernor};

#[test]
fn test_entropy_filter_and_rollback() {
    let governor = ThermodynamicGovernor::new(0.01); // Very strict epsilon

    // Uniform state (low entropy)
    let current = vec![
        Node { theta: 0.1, e: 1.0, ec: 0.5, _padding: 0 },
        Node { theta: 0.1, e: 1.0, ec: 0.5, _padding: 0 },
        Node { theta: 0.1, e: 1.0, ec: 0.5, _padding: 0 },
        Node { theta: 0.1, e: 1.0, ec: 0.5, _padding: 0 },
    ];

    // Valid transition (stable)
    let next_valid = vec![
        Node { theta: 0.11, e: 1.0, ec: 0.5, _padding: 0 },
        Node { theta: 0.11, e: 1.0, ec: 0.5, _padding: 0 },
        Node { theta: 0.11, e: 1.0, ec: 0.5, _padding: 0 },
        Node { theta: 0.11, e: 1.0, ec: 0.5, _padding: 0 },
    ];

    assert!(governor.validate_transition(&current, &next_valid).is_ok());

    // Highly dispersed state (high entropy spike)
    let next_chaotic = vec![
        Node { theta: -0.9, e: 1.0, ec: 0.5, _padding: 0 },
        Node { theta: -0.3, e: 1.0, ec: 0.5, _padding: 0 },
        Node { theta: 0.3, e: 1.0, ec: 0.5, _padding: 0 },
        Node { theta: 0.9, e: 1.0, ec: 0.5, _padding: 0 },
    ];

    let result = governor.validate_transition(&current, &next_chaotic);
    assert!(result.is_err(), "Chaotic transition must trigger thermodynamic rollback");
    println!("Successfully caught violation: {}", result.unwrap_err());
}
