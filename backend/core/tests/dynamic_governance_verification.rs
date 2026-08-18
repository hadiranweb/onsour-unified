use core_engine::{Node, ThermodynamicGovernor, SystemMetrics, LogicalTimestamp};

#[test]
fn test_dynamic_epsilon_adaptation_and_robustness() {
    let mut governor = ThermodynamicGovernor::new(0.1, 0.005, 0.25, 0.3).unwrap();
    let tick = LogicalTimestamp::new(0, 100);

    // Test constructor invariant validation
    assert!(ThermodynamicGovernor::new(0.1, 0.2, 0.5, 0.3).is_err(), "min > base must fail");
    assert!(ThermodynamicGovernor::new(0.5, 0.1, 0.2, 0.3).is_err(), "base > max must fail");
    assert!(ThermodynamicGovernor::new(0.1, -0.1, 0.5, 0.3).is_err(), "negative min must fail");
    assert!(ThermodynamicGovernor::new(0.1, 0.01, 0.5, 1.5).is_err(), "alpha > 1.0 must fail");

    // Normal metrics
    let normal_metrics = SystemMetrics::new(0.2, 0.2, 10.0, tick);
    for _ in 0..10 { governor.compute_epsilon(&normal_metrics, tick); }
    let eps_normal = governor.compute_epsilon(&normal_metrics, tick);
    println!("Epsilon Normal: {:.4}", eps_normal);
    assert!((eps_normal - 0.090).abs() < 0.01);

    // Stressed metrics
    let stressed_metrics = SystemMetrics::new(0.95, 0.9, 250.0, tick);
    for _ in 0..10 { governor.compute_epsilon(&stressed_metrics, tick); }
    let eps_stressed = governor.compute_epsilon(&stressed_metrics, tick);
    println!("Epsilon Stressed: {:.4}", eps_stressed);
    assert!(eps_stressed < eps_normal);

    // Test NaN / Stale fail-safe
    let corrupted_metrics = SystemMetrics {
        cpu_load: f32::NAN,
        memory_pressure: 0.5,
        network_latency_ms: 50.0,
        logical_timestamp: tick,
    };
    let eps_corrupted = governor.compute_epsilon(&corrupted_metrics, tick);
    assert_eq!(eps_corrupted, governor.min_epsilon);

    let stale_metrics = SystemMetrics::new(0.2, 0.2, 10.0, LogicalTimestamp::new(0, 10));
    let eps_stale = governor.compute_epsilon(&stale_metrics, LogicalTimestamp::new(0, 100));
    assert_eq!(eps_stale, governor.min_epsilon);
}

#[test]
fn epsilon_is_bounded_for_valid_and_extreme_metrics() {
    let mut gov = ThermodynamicGovernor::new(0.1, 0.005, 0.25, 0.3).unwrap();
    let tick = LogicalTimestamp::new(0, 100);

    let cases = [
        SystemMetrics::new(0.0, 0.0, 0.0, tick),
        SystemMetrics::new(1.0, 1.0, 10_000.0, tick),
        SystemMetrics::new(-1.0, 5.0, -100.0, tick),
    ];

    for m in cases {
        let e = gov.compute_epsilon(&m, tick);
        assert!(e.is_finite() && e >= 0.005 && e <= 0.25);
    }
}
