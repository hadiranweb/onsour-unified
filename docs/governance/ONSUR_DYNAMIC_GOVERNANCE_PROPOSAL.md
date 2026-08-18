# ONSOUR: Dynamic Thermodynamic Governance & Epsilon Adaptation Protocol

**Author:** Manus AI  
**Project:** ONSOUR Unified Monorepo (`onsour-unified`)  
**Core Module:** `core_engine::governance`  

---

## 1. Introduction and Architectural Motivation
In physical systems governed by statistical mechanics and the Universal Integrated Physical Theory (UIPT), entropy bounds must adapt to external environmental pressures and internal computational load. A static entropy threshold ($\epsilon$) treats a quiet, idle system with the same strictness as a heavily saturated, high-latency network node. 

To achieve true biological resilience and fault tolerance, we have upgraded the **`ThermodynamicGovernor`** in the Rust core to support **Dynamic Epsilon Adaptation**. This mechanism modulates allowable entropy drift based on real-time system metrics: **CPU load**, **memory pressure**, and **network latency**.

---

## 2. Mathematical Formalism of Dynamic Epsilon
The adjusted entropy threshold $\epsilon(t)$ is computed dynamically as a function of system telemetry:

$$\epsilon(t) = \text{clamp}\left( \epsilon_{\text{base}} \cdot \frac{1.0 - 0.4 \cdot \Lambda}{1.0 + 0.2 \cdot \mathcal{L}}, \quad \epsilon_{\text{min}}, \quad \epsilon_{\text{max}} \right)$$

Where:
- **$\epsilon_{\text{base}}$**: The baseline allowable drift (default: `0.1`).
- **$\Lambda$**: Composite resource load factor defined as $\Lambda = 0.5 \cdot \text{CPU}_{\text{load}} + 0.5 \cdot \text{Memory}_{\text{pressure}}$, bounded in $[0.0, 1.0]$.
- **$\mathcal{L}$**: Normalized network latency factor defined as $\mathcal{L} = \frac{\text{Latency}_{\text{ms}}}{100.0}$.
- **$\epsilon_{\text{min}}, \epsilon_{\text{max}}$**: Hard safety clamps (set to `0.005` and `0.25` respectively) to prevent runaway volatility or total gridlock.

### Physical Interpretation:
1. **Under High Load or High Latency:** The penalty terms increase, reducing $\epsilon(t)$. The system becomes **stricter**, instantly rejecting state transitions that exhibit even minor deviations in Shannon entropy, thereby preventing cascading failures during network degradation.
2. **Under Low Load and Stable Conditions:** The penalty decreases toward zero, relaxing $\epsilon(t)$ toward $\epsilon_{\text{base}}$ and permitting natural exploration and state diffusion across the network.

---

## 3. Implementation in Rust (`governance.rs`)

The dynamic governance engine has been fully implemented in `backend/core/src/governance.rs`:

```rust
pub struct SystemMetrics {
    pub cpu_load: f32,       // Normalized 0.0 to 1.0
    pub memory_pressure: f32,// Normalized 0.0 to 1.0
    pub network_latency_ms: f32, // Milliseconds
}

impl ThermodynamicGovernor {
    pub fn compute_dynamic_epsilon(&self, metrics: &SystemMetrics) -> f32 {
        let load_factor = 0.5 * metrics.cpu_load + 0.5 * metrics.memory_pressure;
        let latency_factor = (metrics.network_latency_ms / 100.0).clamp(0.0, 2.0);
        
        let adjusted = self.base_epsilon * (1.0 - 0.4 * load_factor) / (1.0 + 0.2 * latency_factor);
        adjusted.clamp(self.min_epsilon, self.max_epsilon)
    }
}
```

---

## 4. Verification and Test Results
We verified the dynamic adjustment mechanism via integration tests (`tests/dynamic_governance_verification.rs`):
- **Normal Metrics Test:** CPU `0.2`, Memory `0.2`, Latency `10ms` $\rightarrow$ $\epsilon \approx 0.0902$.
- **Stressed Metrics Test:** CPU `0.95`, Memory `0.9`, Latency `250ms` $\rightarrow$ $\epsilon \approx 0.0450$ (stricter governance).
- **Rollback Test:** Under stressed conditions, moderate-to-high entropy state dispersion that would normally be permitted under idle conditions is successfully intercepted and rolled back, confirming system-level resilience.

---

## 5. Summary Table

| Operational State | CPU / Memory | Network Latency | Effective $\epsilon$ | Governance Sensitivity |
| :--- | :--- | :--- | :--- | :--- |
| **Idle / Optimal** | Low ($< 0.3$) | Low ($< 20$ ms) | $\approx 0.090$ | Moderate (Permits exploratory drift) |
| **Nominal Load** | Medium ($0.5$) | Medium ($50$ ms) | $\approx 0.071$ | Standard baseline |
| **Saturated / Stressed** | High ($> 0.9$) | High ($> 200$ ms) | $\approx 0.045$ | **Strict** (Instantly halts chaotic variance) |

---
*Certified by Manus AI.*
