# ONSOUR Ecosystem: Foundation Level 1 Development Guide

## 1. Overview
Foundation Level 1 establishes the bedrock architecture for the ONSOUR Unified Ecosystem. Rather than a mere collection of repositories, this setup provides a cohesive **Master Monorepo (`onsour-unified`)** complete with workspace configurations, automated build pipelines, and a declarative module registry.

## 2. Architectural Components

### A. Backend Workspace (`backend/`)
Configured as a standard **Cargo Workspace**, allowing multiple Rust crates (core runtime, services, MCP tools) to share dependencies, compilation targets, and optimization profiles.
- **`backend/core/rts_core`**: The high-performance Langevin/Ising runtime (SPEC v0.4).

### B. Frontend Workspace (`frontend/` & `agents/`)
Managed via **pnpm Workspaces** and Turborepo, organizing user-facing applications (Remix/Next.js portals) and background worker agents.

### C. The Module Registry (`foundation.yaml`)
To support the "Island" architecture, all platform modules, services, and apps are declared in `foundation.yaml`. This file acts as the single source of truth for dependency resolution and service discovery across the ecosystem.

---

## 3. Developer Workflow & Commands

### Initializing the Environment
To set up the monorepo workspace and ensure all toolchains (including WASM compilation tools) are ready:
```bash
make setup
```

### Building the Tanh-Brain Core into WASM
To compile the high-performance Rust core and automatically distribute the WebAssembly bindings to the frontend package directory:
```bash
make build-core
```

### Running Integrated Tests
To execute the test suite across both Rust backend components and TypeScript packages:
```bash
make test
```
