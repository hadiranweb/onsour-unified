use core_engine::{Node, ThermodynamicGovernor, SystemMetrics};

#[test]
fn test_dynamic_epsilon_adaptation_and_robustness() {
    let mut governor = ThermodynamicGovernor::new(0.1);

    // Test constructor invariant validation
    assert!(ThermodynamicGovernor::new_with_bounds(0.1, 0.2, 0.5).is_err(), "min > base must fail");
    assert!(ThermodynamicGovernor::new_with_bounds(0.5, 0.1, 0.2).is_err(), "base > max must fail");
    assert!(ThermodynamicGovernor::new_with_bounds(0.1, -0.1, 0.5).is_err(), "negative min must fail");

    // Normal metrics
    let normal_metrics = SystemMetrics::new(0.2, 0.2, 10.0);
    // Warm up EMA
    for _ in 0..5 {
        governor.compute_dynamic_epsilon(&normal_metrics);
    }
    let eps_normal = governor.compute_dynamic_epsilon(&normal_metrics);
    println!("Epsilon Normal: {:.4}", eps_normal);
    assert!((eps_normal - 0.090).abs() < 0.005);

    // Stressed metrics
    let stressed_metrics = SystemMetrics::new(0.95, 0.9, 250.0);
    for _ in 0..5 {
        governor.compute_dynamic_epsilon(&stressed_metrics);
    }
    let eps_stressed = governor.compute_dynamic_epsilon(&stressed_metrics);
    println!("Epsilon Stressed: {:.4}", eps_stressed);
    assert!(eps_stressed < eps_normal);

    // Test NaN / Inf fail-safe
    let corrupted_metrics = SystemMetrics {
        cpu_load: f32::NAN,
        memory_pressure: 0.5,
        network_latency_ms: 50.0,
        last_updated_at: std::time::Instant::now(),
    };
    let eps_corrupted = governor.compute_dynamic_epsilon(&corrupted_metrics);
    assert_eq!(eps_corrupted, governor.min_epsilon, "Corrupted NaN telemetry must fallback to min_epsilon");

    // Test transition under stress with snapshot
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

    let res_stressed = governor.validate_transition_with_snapshot(&current, &next_state_chaotic, &stressed_metrics, 42);
    assert!(res_stressed.is_err(), "High entropy drift must be rejected under stressed system conditions");
    
    // Test refresh logic
    let fresh_m = SystemMetrics::new(0.5, 0.5, 20.0);
    assert!(!fresh_m.needs_refresh());
}

#[test]
fn epsilon_is_bounded_for_valid_and_extreme_metrics() {
    let mut gov = ThermodynamicGovernor::new_with_bounds(0.1, 0.005, 0.25).unwrap();

    let cases = [
        SystemMetrics::new(0.0, 0.0, 0.0),
        SystemMetrics::new(1.0, 1.0, 10_000.0),
        SystemMetrics::new(-1.0, 5.0, -100.0),
    ];

    for m in cases {
        let e = gov.compute_dynamic_epsilon(&m);
        assert!(e.is_finite());
        assert!(e >= 0.005);
        assert!(e <= 0.25);
    }
}
