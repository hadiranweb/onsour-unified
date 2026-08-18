use crate::state::Node;
use serde::{Serialize, Deserialize};

/// SystemMetrics using logical sequence ticks for serialization and replayability
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct SystemMetrics {
    pub cpu_load: f32,                    // Normalized 0.0 to 1.0
    pub memory_pressure: f32,            // Normalized 0.0 to 1.0
    pub network_latency_ms: f32,         // Milliseconds (must be >= 0)
    pub timestamp_tick: u64,             // Logical tick for staleness check
}

impl SystemMetrics {
    pub fn new(cpu_load: f32, memory_pressure: f32, network_latency_ms: f32, timestamp_tick: u64) -> Self {
        let cpu_load = cpu_load.clamp(0.0, 1.0);
        let memory_pressure = memory_pressure.clamp(0.0, 1.0);
        let network_latency_ms = network_latency_ms.max(0.0);
        
        SystemMetrics {
            cpu_load,
            memory_pressure,
            network_latency_ms,
            timestamp_tick,
        }
    }

    /// Check if telemetry is stale based on tick difference (e.g. max 50 ticks drift)
    pub fn is_stale(&self, current_tick: u64, max_drift: u64) -> bool {
        current_tick > self.timestamp_tick && (current_tick - self.timestamp_tick) > max_drift
    }
}

/// Authoritative snapshot for deterministic replay
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct GovernanceSnapshot {
    pub epoch_id: u64,
    pub authoritative_epsilon: f32,
    pub smoothed_load: f32,
    pub smoothed_latency: f32,
}

/// Thermodynamic Governance Engine with Production-Grade Safety, EMA Smoothing, and Authoritative Replay
pub struct ThermodynamicGovernor {
    pub base_epsilon: f32,
    pub min_epsilon: f32,
    pub max_epsilon: f32,
    
    ema_alpha: f32,
    smoothed_load_factor: f32,
    smoothed_latency_factor: f32,
}

impl ThermodynamicGovernor {
    pub fn new(base_epsilon: f32) -> Self {
        Self::new_with_bounds(base_epsilon, 0.005, 0.25, 0.3).unwrap()
    }

    pub fn new_with_bounds(
        base_epsilon: f32,
        min_epsilon: f32,
        max_epsilon: f32,
        ema_alpha: f32,
    ) -> Result<Self, &'static str> {
        if !base_epsilon.is_finite() || !min_epsilon.is_finite() || !max_epsilon.is_finite() || !ema_alpha.is_finite() {
            return Err("GOVERNOR_INIT_ERROR: Parameters must be finite numbers.");
        }
        if min_epsilon <= 0.0 || min_epsilon > base_epsilon || base_epsilon > max_epsilon {
            return Err("GOVERNOR_INIT_ERROR: Invalid bounds. Expected 0 < min_epsilon <= base_epsilon <= max_epsilon");
        }
        if ema_alpha <= 0.0 || ema_alpha > 1.0 {
            return Err("GOVERNOR_INIT_ERROR: EMA alpha must satisfy 0.0 < alpha <= 1.0");
        }

        Ok(Self {
            base_epsilon,
            min_epsilon,
            max_epsilon,
            ema_alpha,
            smoothed_load_factor: 0.0,
            smoothed_latency_factor: 0.0,
        })
    }

    /// Computes dynamic epsilon with robust numerical guards, telemetry validation, and EMA smoothing
    pub fn compute_dynamic_epsilon(&mut self, metrics: &SystemMetrics, current_tick: u64) -> f32 {
        // Fail-safe under corrupt telemetry or stale data (> 50 ticks drift)
        if !metrics.cpu_load.is_finite()
            || !metrics.memory_pressure.is_finite()
            || !metrics.network_latency_ms.is_finite()
            || metrics.is_stale(current_tick, 50)
        {
            return self.min_epsilon; 
        }

        let cpu = metrics.cpu_load.clamp(0.0, 1.0);
        let memory = metrics.memory_pressure.clamp(0.0, 1.0);
        let latency_ms = metrics.network_latency_ms.max(0.0);

        let raw_load = (0.5 * cpu + 0.5 * memory).clamp(0.0, 1.0);
        let raw_latency = (latency_ms / 100.0).clamp(0.0, 2.0);

        self.smoothed_load_factor = 
            self.ema_alpha * raw_load + (1.0 - self.ema_alpha) * self.smoothed_load_factor;
        self.smoothed_latency_factor = 
            self.ema_alpha * raw_latency + (1.0 - self.ema_alpha) * self.smoothed_latency_factor;

        let numerator = (1.0 - 0.4 * self.smoothed_load_factor).clamp(0.1, 1.0);
        let denominator = (1.0 + 0.2 * self.smoothed_latency_factor).clamp(0.5, f32::INFINITY);

        let adjusted = self.base_epsilon * (numerator / denominator);
        adjusted.clamp(self.min_epsilon, self.max_epsilon)
    }

    /// Calculates Shannon entropy of activation states (theta)
    pub fn calculate_entropy(&self, nodes: &[Node]) -> f32 {
        if nodes.is_empty() {
            return 0.0;
        }

        let num_bins = 5;
        let mut bins = vec![0_f32; num_bins];
        let total = nodes.len() as f32;

        for node in nodes {
            let normalized = ((node.theta + 1.0) * 0.5).clamp(0.0, 0.999);
            let bin_idx = (normalized * num_bins as f32) as usize;
            bins[bin_idx] += 1.0;
        }

        let mut entropy = 0.0;
        for count in bins {
            if count > 0.0 {
                let p = count / total;
                entropy -= p * p.ln();
            }
        }

        entropy
    }

    /// Validates state transition and produces an authoritative GovernanceSnapshot for replay
    pub fn validate_transition_with_snapshot(
        &mut self,
        current_state: &[Node],
        next_state: &[Node],
        metrics: &SystemMetrics,
        current_tick: u64,
        epoch_id: u64,
    ) -> Result<GovernanceSnapshot, String> {
        let h_current = self.calculate_entropy(current_state);
        let h_next = self.calculate_entropy(next_state);
        let epsilon = self.compute_dynamic_epsilon(metrics, current_tick);

        if h_next > h_current + epsilon {
            return Err(format!(
                "THERMODYNAMIC_VIOLATION: Entropy change ({:.4}) exceeds dynamic epsilon ({:.4}) under load {:.2}, latency {:.1}ms. Rollback triggered.",
                h_next - h_current,
                epsilon,
                self.smoothed_load_factor,
                self.smoothed_latency_factor
            ));
        }

        Ok(GovernanceSnapshot {
            epoch_id,
            authoritative_epsilon: epsilon,
            smoothed_load: self.smoothed_load_factor,
            smoothed_latency: self.smoothed_latency_factor,
        })
    }

    pub fn validate_transition(&mut self, current_state: &[Node], next_state: &[Node]) -> Result<(), &'static str> {
        let default_metrics = SystemMetrics::new(0.5, 0.4, 20.0, 0);
        self.validate_transition_with_snapshot(current_state, next_state, &default_metrics, 0, 0)
            .map(|_| ())
            .map_err(|_| "THERMODYNAMIC_VIOLATION: Entropy constraint exceeded under default metrics. Rollback triggered.")
    }
}
