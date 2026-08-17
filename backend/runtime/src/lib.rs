use core_engine::StochasticState;
use rayon::prelude::*;

/// A parallel execution domain (Island) for processing multiple stochastic units.
pub struct Island {
    pub states: Vec<StochasticState>,
}

impl Island {
    pub fn new(num_units: usize, r: f64, d: f64) -> Self {
        let states = vec![StochasticState::new(r, d); num_units];
        Self { states }
    }

    /// Parallel processing of all states within the island.
    pub fn process(&mut self, dt: f64, noise: &[f64]) {
        self.states.par_iter_mut().enumerate().for_each(|(i, state)| {
            state.step(dt, noise[i]);
        });
    }
}
