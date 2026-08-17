/// The standard interface for any module (Island) to plug into the ONSOUR Level 1 Infrastructure.
pub trait OnsourModule {
    /// Returns the unique identifier of the module.
    fn id(&self) -> &str;

    /// Initializes the module with its specific stochastic parameters.
    fn init(&mut self);

    /// Executed during the Synaptic Hub convergence phase.
    fn on_converge(&mut self, global_state: &[f64]);

    /// Executed during every execution epoch.
    fn update(&mut self, dt: f64);
}
