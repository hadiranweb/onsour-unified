use core_engine::{Node, ThermodynamicGovernor, SystemMetrics};

#[test]
fn test_dynamic_epsilon_adaptation_and_robustness() {
    let mut governor = ThermodynamicGovernor::new(0.1);

    // Test constructor invariant validation
    assert!(ThermodynamicGovernor::new_with_bounds(0.1, 0.2, 0.5).is_err(), "min > base must fail");
    assert!(ThermodynamicGovernor::new_with_bounds(0.5, 0.1, 0.2).is_err(), "base > max must fail");
    assert!(ThermodynamicGovernor::new_with_bounds(0.1, -0.1, 0.5).is_err(), "negative min must fail");

    // Normal metrics
    let normal_metrics = SystemMetrics {
        cpu_load: 0.2,
        memory_pressure: 0.2,
        network_latency_ms: 10.0,
    };

    // Stressed metrics
    let stressed_metrics = SystemMetrics {
        cpu_load: 0.95,
        memory_pressure: 0.9,
        network_latency_ms: 250.0,
    };

    let eps_normal = governor.compute_dynamic_epsilon(&normal_metrics);
    let eps_stressed = governor.compute_dynamic_epsilon(&stressed_metrics);

    println!("Epsilon Normal: {}, Epsilon Stressed: {}", eps_normal, eps_stressed);
    assert!(eps_stressed < eps_normal);

    // Test NaN / Inf fail-safe
    let corrupted_metrics = SystemMetrics {
        cpu_load: f32::NAN,
        memory_pressure: 0.5,
        network_latency_ms: 50.0,
    };
    let eps_corrupted = governor.compute_dynamic_epsilon(&corrupted_metrics);
    assert_eq!(eps_corrupted, governor.min_epsilon, "Corrupted NaN telemetry must fallback to min_epsilon");

    // Test transition under stress
    let current = vec![
        Node { theta: 0.1, e: 1.0, ec: 0.5, _padding: 0 },
        Node { theta: 0.1, e: 1.0, ec: 0.5, _padding: 0 },
        Node { theta: 0.1, e: 1.0, ec: 0.5, _padding: 0 },
        Node { theta: 0.1, e: 1.0, ec: 0.5, _padding: 0 },
    ];

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
