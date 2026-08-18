# ONSOUR Unified: Level 1 Infrastructure
**The Scientific Foundation for Digital Ecosystems**

This repository is the **Single Source of Truth** for the ONSOUR ecosystem. It provides a clean, high-performance, and scientifically verified infrastructure for building complex stochastic networks based on the **Universal Integrated Physical Theory (UIPT)**.

## 🚀 Key Features
- **Deterministic Rust Core:** Bit-exact reproducibility across parallel threads.
- **Island Runtime:** Lock-free, data-parallel execution with double buffering.
- **Hardware Alignment:** 32-byte aligned memory layout to eliminate cache-line straddling.
- **Unified Manifest:** Integrated orchestration via `foundation.yaml` and `Makefile`.

## 📂 Project Structure
- `backend/core`: The Langevin/Tanh-Brain mathematical engine.
- `backend/runtime`: The parallel execution engine (Islands).
- `backend/synaptic-hub`: The central orchestration hub.
- `docs/`: Technical specifications, architectural defenses, and verification reports.

## 🛠️ Getting Started
```bash
# Setup dependencies
make setup

# Run scientific verification tests
make test

# Build the native hub
make build-runtime

# Build the core for Web (WASM)
make build-core
```

## 🔬 Scientific Verification
The ONSOUR core is verified against:
1. **Layout Integrity:** Ensuring zero cache-line straddling.
2. **Numerical Determinism:** Ensuring bit-identical results in multi-threaded environments.
3. **Statistical Moments:** Ensuring convergence to the Boltzmann distribution.

---
*Developed by hadiranweb and Manus AI.*
