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

/// Thermodynamic Governance Engine with Dynamic Epsilon Adaptation & Robust Invariants
pub struct ThermodynamicGovernor {
    pub base_epsilon: f32,
    pub min_epsilon: f32,
    pub max_epsilon: f32,
    // EMA smoothing state
    smoothed_load: f32,
    smoothed_latency: f32,
    alpha: f32, // Smoothing factor for EMA
}

impl ThermodynamicGovernor {
    /// Creates a new governor with validated invariants
    pub fn new(base_epsilon: f32) -> Self {
        Self::new_with_bounds(base_epsilon, 0.005, 0.25).unwrap()
    }

    pub fn new_with_bounds(base_epsilon: f32, min_epsilon: f32, max_epsilon: f32) -> Result<Self, &'static str> {
        if min_epsilon <= 0.0 || min_epsilon > base_epsilon || base_epsilon > max_epsilon {
            return Err("GOVERNOR_INIT_ERROR: Invalid epsilon bounds. Must satisfy 0 < min_epsilon <= base_epsilon <= max_epsilon");
        }
        if !min_epsilon.is_finite() || !base_epsilon.is_finite() || !max_epsilon.is_finite() {
            return Err("GOVERNOR_INIT_ERROR: Epsilon bounds must be finite numbers.");
        }

        Ok(Self {
            base_epsilon,
            min_epsilon,
            max_epsilon,
            smoothed_load: 0.5,
            smoothed_latency: 20.0,
            alpha: 0.2,
        })
    }

    /// Computes dynamic epsilon with strict input validation, NaN/Inf checks, and EMA smoothing.
    pub fn compute_dynamic_epsilon(&mut self, metrics: &SystemMetrics) -> f32 {
        if !metrics.cpu_load.is_finite()
            || !metrics.memory_pressure.is_finite()
            || !metrics.network_latency_ms.is_finite()
        {
            return self.min_epsilon;
        }

        let cpu = metrics.cpu_load.clamp(0.0, 1.0);
        let memory = metrics.memory_pressure.clamp(0.0, 1.0);
        let latency_ms = metrics.network_latency_ms.max(0.0);

        let raw_load = 0.5 * cpu + 0.5 * memory;

        self.smoothed_load = self.alpha * raw_load + (1.0 - self.alpha) * self.smoothed_load;
        self.smoothed_latency = self.alpha * latency_ms + (1.0 - self.alpha) * self.smoothed_latency;

        let load_factor = self.smoothed_load;
        let latency_factor = (self.smoothed_latency / 100.0).clamp(0.0, 2.0);

        let adjusted = self.base_epsilon * (1.0 - 0.4 * load_factor) / (1.0 + 0.2 * latency_factor);

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

    pub fn validate_transition_with_metrics(
        &mut self,
        current_state: &[Node],
        next_state: &[Node],
        metrics: &SystemMetrics,
    ) -> Result<f32, String> {
        let h_current = self.calculate_entropy(current_state);
        let h_next = self.calculate_entropy(next_state);
        let epsilon = self.compute_dynamic_epsilon(metrics);

        if h_next > h_current + epsilon {
            return Err(format!(
                "THERMODYNAMIC_VIOLATION: Entropy change ({:.4}) exceeds dynamic epsilon ({:.4}) under load {:.2}, latency {:.1}ms. Rollback triggered.",
                h_next - h_current,
                epsilon,
                self.smoothed_load,
                self.smoothed_latency
            ));
        }

        Ok(epsilon)
    }

    pub fn validate_transition(&mut self, current_state: &[Node], next_state: &[Node]) -> Result<(), &'static str> {
        let default_metrics = SystemMetrics::default();
        self.validate_transition_with_metrics(current_state, next_state, &default_metrics)
            .map(|_| ())
            .map_err(|_| "THERMODYNAMIC_VIOLATION: Entropy constraint exceeded under default metrics. Rollback triggered.")
    }
}
