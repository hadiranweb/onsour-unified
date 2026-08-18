# ONSOUR Ecosystem: Cross-Language "Glue Code" Strategy

## 1. Introduction
The ONSOUR Unified Monorepo integrates diverse technologies: Python for scientific discovery and research (`theory/`), Rust for high-performance low-level runtime (`backend/core/`), and TypeScript/Next.js/Remix for user interfaces (`frontend/`). 

To ensure that changes in theoretical models (e.g., Ising equation parameters) automatically propagate to backend engines and frontend dashboards, a robust **Glue Code Strategy** is required.

## 2. Integration Channels & Bridges

```text
[ Python Research (Track A) ]
         │
         │  (Shared Test Vectors / JSON Manifest)
         ▼
[ Rust Engine (Track B/C) ] ──(WASM / C ABI)──> [ Frontend (Next.js / Remix) ]
         │
         └──(Model Context Protocol - MCP)──> [ AI Agents & Services ]
```

### 2.1 Python to Rust (Track C Verification)
- **Mechanism:** The Python reference implementation (`theory/python_reference/`) exports fixed test vectors (`test_vectors.json`) and SHA256 manifests.
- **Glue Code:** CI/CD pipelines run automated equivalence tests (`cargo test --test numeric_equivalence`) on every commit to `theory/` or `backend/core/`. If divergence exceeds $10^{-4}$, the build fails immediately.

### 2.2 Rust to Frontend (WASM & C ABI)
- **Mechanism:** The high-performance Rust core (`backend/core/rts_core`) compiles into WebAssembly (WASM) for client-side execution in dashboards (`frontend/apps/portal/`) or native libraries via C ABI.
- **Glue Code:** TypeScript type definitions are automatically generated from Rust structs using `wasm-bindgen` and `typescript-definitions`, ensuring zero-cost type safety across the language boundary.

### 2.3 Agentic & Tool Integration (MCP)
- **Mechanism:** Specialized services like `casio-plus` act as Native Model Context Protocol (MCP) servers.
- **Glue Code:** Exposes Rust core simulation steps as standard MCP tools, allowing AI agents (`frontend/` or external bots) to trigger state transitions and query order parameters $\theta$ programmatically.

## 3. Monorepo CI/CD Enforcement
1. **Linting & Formatting:** Unified Prettier and Rustfmt rules across the workspace.
2. **Dependency Locking:** Cargo workspaces and pnpm workspaces configured to prevent version drift.
3. **Automated Synchronization:** Changes in `theory/rts_reference_frozen_v0_4.py` trigger automated regression suites against `backend/core/`.
