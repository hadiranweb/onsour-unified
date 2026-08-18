use crate::state::Node;

/// System metrics feeding into the Dynamic Thermodynamic Governor
#[derive(Debug, Clone, Copy)]
pub struct SystemMetrics {
    pub cpu_load: f32,       // Normalized 0.0 to 1.0
    pub memory_pressure: f32,// Normalized 0.0 to 1.0
    pub network_latency_ms: f32, // Milliseconds
}

impl Default for SystemMetrics {
    fn default() -> Self {
        Self {
            cpu_load: 0.5,
            memory_pressure: 0.4,
            network_latency_ms: 20.0,
        }
    }
}

/// Thermodynamic Governance Engine with Dynamic Epsilon Adaptation
/// Enforces the entropy constraint: H(S_{t+1}) <= H(S_t) + epsilon(load, latency)
pub struct ThermodynamicGovernor {
    pub base_epsilon: f32,
    pub min_epsilon: f32,
    pub max_epsilon: f32,
}

impl ThermodynamicGovernor {
    pub fn new(base_epsilon: f32) -> Self {
        Self {
            base_epsilon,
            min_epsilon: 0.005,
            max_epsilon: 0.25,
        }
    }

    /// Computes dynamic epsilon based on real-time system metrics.
    /// High load or high latency tightens epsilon (reduces allowable drift to prevent systemic instability).
    /// Low load relaxes epsilon slightly to permit rapid state exploration.
    pub fn compute_dynamic_epsilon(&self, metrics: &SystemMetrics) -> f32 {
        // Load penalty factor
        let load_factor = 0.5 * metrics.cpu_load + 0.5 * metrics.memory_pressure;
        
        // Latency penalty factor (normalized around 100ms)
        let latency_factor = (metrics.network_latency_ms / 100.0).clamp(0.0, 2.0);

        // Combined penalty: higher load/latency -> smaller epsilon (stricter governance)
        // Epsilon = Base * (1.0 - 0.4 * load) / (1.0 + 0.2 * latency)
        let adjusted = self.base_epsilon * (1.0 - 0.4 * load_factor) / (1.0 + 0.2 * latency_factor);

        adjusted.clamp(self.min_epsilon, self.max_epsilon)
    }

    /// Calculates the Shannon-like entropy of the activation states (theta) across nodes.
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

    /// Validates if the state transition from t to t+1 satisfies the governance rules using dynamic metrics.
    pub fn validate_transition_with_metrics(
        &self,
        current_state: &[Node],
        next_state: &[Node],
        metrics: &SystemMetrics,
    ) -> Result<f32, String> {
        let h_current = self.calculate_entropy(current_state);
        let h_next = self.calculate_entropy(next_state);
        let epsilon = self.compute_dynamic_epsilon(metrics);

        if h_next > h_current + epsilon {
            return Err(format!(
                "THERMODYNAMIC_VIOLATION: Entropy change ({:.4}) exceeds dynamic epsilon ({:.4}) at CPU load {:.2}, latency {:.1}ms. Rollback triggered.",
                h_next - h_current,
                epsilon,
                metrics.cpu_load,
                metrics.network_latency_ms
            ));
        }

        Ok(epsilon)
    }

    /// Backward compatible validation method using default metrics
    pub fn validate_transition(&self, current_state: &[Node], next_state: &[Node]) -> Result<(), &'static str> {
        let default_metrics = SystemMetrics::default();
        self.validate_transition_with_metrics(current_state, next_state, &default_metrics)
            .map(|_| ())
            .map_err(|_| "THERMODYNAMIC_VIOLATION: Entropy constraint exceeded under default metrics. Rollback triggered.")
    }
}
