use core_engine::{Node, ThermodynamicGovernor, SystemMetrics};

#[test]
fn test_dynamic_epsilon_adaptation() {
    let governor = ThermodynamicGovernor::new(0.1);

    // Normal metrics
    let normal_metrics = SystemMetrics {
        cpu_load: 0.2,
        memory_pressure: 0.2,
        network_latency_ms: 10.0,
    };

    // Stressed metrics (high CPU & latency)
    let stressed_metrics = SystemMetrics {
        cpu_load: 0.95,
        memory_pressure: 0.9,
        network_latency_ms: 250.0,
    };

    let eps_normal = governor.compute_dynamic_epsilon(&normal_metrics);
    let eps_stressed = governor.compute_dynamic_epsilon(&stressed_metrics);

    println!("Epsilon Normal: {}, Epsilon Stressed: {}", eps_normal, eps_stressed);

    // Stressed epsilon must be significantly tighter (smaller) than normal epsilon
    assert!(eps_stressed < eps_normal, "Stressed epsilon must be stricter under high load and latency");

    // Test transition under stress with a larger entropy spike
    let current = vec![
        Node { theta: 0.1, e: 1.0, ec: 0.5, _padding: 0 },
        Node { theta: 0.1, e: 1.0, ec: 0.5, _padding: 0 },
        Node { theta: 0.1, e: 1.0, ec: 0.5, _padding: 0 },
        Node { theta: 0.1, e: 1.0, ec: 0.5, _padding: 0 },
    ];

    // High dispersion that creates an entropy spike exceeding eps_stressed (0.045)
    let next_state_chaotic = vec![
        Node { theta: -0.9, e: 1.0, ec: 0.5, _padding: 0 },
        Node { theta: -0.5, e: 1.0, ec: 0.5, _padding: 0 },
        Node { theta: 0.5, e: 1.0, ec: 0.5, _padding: 0 },
        Node { theta: 0.9, e: 1.0, ec: 0.5, _padding: 0 },
    ];

    let res_stressed = governor.validate_transition_with_metrics(&current, &next_state_chaotic, &stressed_metrics);
    assert!(res_stressed.is_err(), "High entropy drift must be rejected under stressed system conditions");
    println!("Successfully rejected under stress: {}", res_stressed.unwrap_err());
}
