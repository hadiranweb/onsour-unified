use serde::{Serialize, Deserialize};

/// The fundamental state of a stochastic unit in the ONSOUR ecosystem.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct StochasticState {
    pub x: f64,
    pub r: f64,
    pub d: f64,
}

impl StochasticState {
    pub fn new(r: f64, d: f64) -> Self {
        Self { x: 0.0, r, d }
    }

    /// Optimized Langevin integration step.
    /// dx = (2rx - 4x^3)dt + sqrt(2Ddt) * eta
    pub fn step(&mut self, dt: f64, eta: f64) {
        let force = 2.0 * self.r * self.x - 4.0 * self.x.powi(3);
        let stochastic = (2.0 * self.d * dt).sqrt() * eta;
        self.x += force * dt + stochastic;
    }

    /// Corrected Kramers frequency for Level 1 stability.
    pub fn omega(&self) -> f64 {
        2.0 * self.r.sqrt()
    }
}
