# Tanh-Brain Mathematical Core Architecture

## 1. Executive Summary
The Tanh-Brain architecture is a theoretical and computational framework designed for **Always-On, Low-Latency autonomous agents**. It is grounded in the physics of **Phase Transitions** and **Spontaneous Symmetry Breaking**, utilizing a nonlinear mapping between system energy and an order parameter to govern behavior.

## 2. Fundamental Equation
The core of the system is the **Mean-Field Ising-type equation**, which maps internal energy states to a bounded behavioral output:

$$\theta(t+1) = \tanh\left(\frac{E(t) - E_c}{E_c} + \sum_{j \in \text{incoming}(i)} J_{ji} \theta_j(t - \tau_{ji})\right)$$

### Key Components:
*   **$\theta$ (Order Parameter):** The state of the agent, bounded in $[-1, 1]$. It represents the degree of phase asymmetry (e.g., decision state, activity level).
*   **$E$ (System Energy):** The instantaneous energy or input signal.
*   **$E_c$ (Critical Energy):** The threshold scale where the system transitions from a symmetric state ($\theta \approx 0$) to a broken-symmetry state ($\theta \approx \pm 1$).
*   **$J_{ji}$ (Interaction Matrix):** The coupling weights between nodes in a sparse graph.
*   **$\tau_{ji}$ (Temporal Lag):** Time delays in signal propagation across the network.

## 3. Two-Channel Architecture ($\alpha / \beta$)
The system processes information through two distinct pathways:
*   **Excitatory Channel ($\alpha$):** Derived as $\alpha = (\theta + 1) / 2$, promoting transitions toward higher energy states or positive polarization.
*   **Inhibitory Channel ($\beta$):** Opposes transitions or restores symmetry, ensuring stability and preventing runaway excitation.

## 4. Hierarchical Energy Structure
Tanh-Brain operates across multiple scales:
1.  **Global Equilibrium ($E_0^{\text{global}}$):** Defines the macro-state of the agent.
2.  **Local Equilibrium ($E_0^{\text{local}}$):** Governs fast, reactive transitions within subsystems.
This hierarchy allows for **fractal-like dynamics**, where local nodes can reach critical states without destabilizing the entire system.

## 5. Implementation Strategy: Track B/C
To ensure the mathematical "truth" discovered in Python (Track A) is preserved in real-world deployment, the architecture enforces:
*   **Track B (Resource Validation):** Rust-based runtime (`rts_core`) for sub-microsecond latency and minimal memory footprint (< 32 bytes per node).
*   **Track C (Numeric Equivalence):** Strict verification that the Rust implementation reproduces Python reference trajectories within a threshold of $10^{-5}$.

## 6. Physical Foundations
*   **Landau-Ginzburg Potential:** The system follows a double-well potential $V(\theta) = -r\theta^2 + u\theta^4$, where $r$ is controlled by the energy ratio $(E-E_c)/E_c$.
*   **Stochastic Dynamics:** Real-world noise is handled via Langevin dynamics, allowing the agent to escape local minima through thermal activation or quantum-like tunneling (modeled via Gamow factors in advanced versions).
