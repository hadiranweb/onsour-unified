use crate::state::Node;
use serde::{Serialize, Deserialize};
use std::fmt;

/// Logical timestamp for deterministic serialization and replay
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct LogicalTimestamp {
    pub epoch_number: u64,
    pub tick_within_epoch: u32,
}

impl LogicalTimestamp {
    pub fn new(epoch: u64, tick: u32) -> Self {
        Self { epoch_number: epoch, tick_within_epoch: tick }
    }

    pub fn is_stale(&self, current: LogicalTimestamp, threshold: u32) -> bool {
        if self.epoch_number < current.epoch_number {
            return true;
        }
        if self.epoch_number == current.epoch_number {
            return current.tick_within_epoch.saturating_sub(self.tick_within_epoch) > threshold;
        }
        false // Telemetry from the future
    }
}

impl fmt::Display for LogicalTimestamp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "e:{},t:{}", self.epoch_number, self.tick_within_epoch)
    }
}

/// System telemetry with logical timestamp and defensive sanitization
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct SystemMetrics {
    pub cpu_load: f32,
    pub memory_pressure: f32,
    pub network_latency_ms: f32,
    pub logical_timestamp: LogicalTimestamp,
}

impl SystemMetrics {
    pub fn new(cpu: f32, mem: f32, latency: f32, tick: LogicalTimestamp) -> Self {
        Self {
            cpu_load: cpu.clamp(0.0, 1.0),
            memory_pressure: mem.clamp(0.0, 1.0),
            network_latency_ms: latency.max(0.0),
            logical_timestamp: tick,
        }
    }

    pub fn is_finite(&self) -> bool {
        self.cpu_load.is_finite() && self.memory_pressure.is_finite() && self.network_latency_ms.is_finite()
    }
}

/// Authoritative snapshot for exact replay
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct GovernanceSnapshot {
    pub epoch: u64,
    pub authoritative_epsilon: f32,
    pub current_dispersion: f32,
    pub candidate_dispersion: f32,
    pub accepted: bool,
}

/// Thermodynamic Governance Engine
pub struct ThermodynamicGovernor {
    pub base_epsilon: f32,
    pub min_epsilon: f32,
    pub max_epsilon: f32,
    pub ema_alpha: f32,
    pub stale_threshold: u32,
    
    smoothed_load: f32,
    smoothed_latency: f32,
}

impl ThermodynamicGovernor {
    pub fn new(base: f32, min: f32, max: f32, alpha: f32) -> Result<Self, &'static str> {
        if !base.is_finite() || !min.is_finite() || !max.is_finite() || !alpha.is_finite() {
            return Err("GOVERNOR_INIT: Parameters must be finite.");
        }
        if min <= 0.0 || min > base || base > max {
            return Err("GOVERNOR_INIT: Invalid bounds. 0 < min <= base <= max");
        }
        if alpha <= 0.0 || alpha > 1.0 {
            return Err("GOVERNOR_INIT: Alpha must be in (0, 1]");
        }

        Ok(Self {
            base_epsilon: base,
            min_epsilon: min,
            max_epsilon: max,
            ema_alpha: alpha,
            stale_threshold: 10,
            smoothed_load: 0.5,
            smoothed_latency: 20.0,
        })
    }

    pub fn with_base(base: f32) -> Self {
        Self::new(base, 0.005, 0.25, 0.3).unwrap()
    }

    /// Computes authoritative epsilon for the current state
    pub fn compute_epsilon(&mut self, metrics: &SystemMetrics, current_tick: LogicalTimestamp) -> f32 {
        if !metrics.is_finite() || metrics.logical_timestamp.is_stale(current_tick, self.stale_threshold) {
            return self.min_epsilon;
        }

        let raw_load = (0.5 * metrics.cpu_load + 0.5 * metrics.memory_pressure).clamp(0.0, 1.0);
        let raw_latency = (metrics.network_latency_ms / 100.0).clamp(0.0, 2.0);

        self.smoothed_load = self.ema_alpha * raw_load + (1.0 - self.ema_alpha) * self.smoothed_load;
        self.smoothed_latency = self.ema_alpha * raw_latency + (1.0 - self.ema_alpha) * self.smoothed_latency;

        let num = (1.0 - 0.4 * self.smoothed_load).clamp(0.1, 1.0);
        let den = (1.0 + 0.2 * self.smoothed_latency).clamp(0.5, f32::INFINITY);

        let adjusted = self.base_epsilon * (num / den);
        adjusted.clamp(self.min_epsilon, self.max_epsilon)
    }

    /// Calculates state dispersion (variance) as a stable alternative to Shannon entropy
    pub fn calculate_dispersion(&self, nodes: &[Node]) -> f32 {
        if nodes.is_empty() { return 0.0; }
        let n = nodes.len() as f64;
        let mean = nodes.iter().map(|n| n.theta as f64).sum::<f64>() / n;
        let variance = nodes.iter().map(|n| {
            let d = n.theta as f64 - mean;
            d * d
        }).sum::<f64>() / n;
        variance as f32
    }

    /// Core decision logic: Accept or Rollback
    pub fn apply_governance(
        &mut self,
        current: &[Node],
        next: &mut [Node],
        metrics: &SystemMetrics,
        current_tick: LogicalTimestamp,
        epoch: u64,
    ) -> GovernanceSnapshot {
        let epsilon = self.compute_epsilon(metrics, current_tick);
        let current_d = self.calculate_dispersion(current);
        let next_d = self.calculate_dispersion(next);

        let accepted = current_d.is_finite() && next_d.is_finite() && next_d <= current_d + epsilon;

        if !accepted {
            next.copy_from_slice(current);
        }

        GovernanceSnapshot {
            epoch,
            authoritative_epsilon: epsilon,
            current_dispersion: current_d,
            candidate_dispersion: next_d,
            accepted,
        }
    }
}
