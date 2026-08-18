use rts_core::StochasticState;
use rand::prelude::*;
use rand_distr::StandardNormal;

#[test]
fn test_boltzmann_equilibrium_moments() {
    let r = 1.0;
    let d = 0.1;
    let dt = 0.01;
    let num_steps = 10000;
    let num_trajectories = 1000;
    
    let mut rng = StdRng::seed_from_u64(42);
    let mut final_states = Vec::with_capacity(num_trajectories);

    for _ in 0..num_trajectories {
        let mut state = StochasticState::new(r, d);
        // Random start to speed up convergence
        state.x = rng.gen_range(-2.0..2.0);
        
        for _ in 0..num_steps {
            let eta: f64 = rng.sample(StandardNormal);
            state.step(dt, eta);
        }
        final_states.push(state.x);
    }

    // Calculate moments
    let mean: f64 = final_states.iter().sum::<f64>() / num_trajectories as f64;
    let variance: f64 = final_states.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / num_trajectories as f64;

    // Theoretical check:
    // For V(x) = -x^2 + x^4, D = 0.1, the minima are at +/- sqrt(0.5) approx +/- 0.707
    // The variance should be centered around these wells.
    
    println!("Mean: {}, Variance: {}", mean, variance);
    
    // Assertions for stability and symmetry
    assert!(mean.abs() < 0.1, "Mean should be close to 0 for symmetric potential, got {}", mean);
    assert!(variance > 0.4 && variance < 0.6, "Variance out of expected range for double-well, got {}", variance);
}
