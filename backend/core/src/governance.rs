use crate::state::Node;

/// Thermodynamic Governance Engine
/// Enforces the entropy constraint: H(S_{t+1}) <= H(S_t) + epsilon
/// And evaluates the potential energy barrier V(S).
pub struct ThermodynamicGovernor {
    pub epsilon: f32, // Allowable entropy drift
}

impl ThermodynamicGovernor {
    pub fn new(epsilon: f32) -> Self {
        Self { epsilon }
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

    /// Validates if the state transition from t to t+1 satisfies the governance rules.
    pub fn validate_transition(&self, current_state: &[Node], next_state: &[Node]) -> Result<(), &'static str> {
        let h_current = self.calculate_entropy(current_state);
        let h_next = self.calculate_entropy(next_state);

        // If entropy increases beyond the allowed epsilon drift, trigger rollback
        if h_next > h_current + self.epsilon {
            return Err("THERMODYNAMIC_VIOLATION: Entropy constraint H(S_t+1) > H(S_t) + epsilon. Rollback triggered.");
        }

        Ok(())
    }
}
