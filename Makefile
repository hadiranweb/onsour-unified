.PHONY: build-core build-runtime setup test clean

# Default target
all: setup build-core build-runtime

# Initial setup
setup:
	@echo "Setting up ONSOUR Foundation Level 1..."
	rustup target add wasm32-unknown-unknown
	cargo install wasm-pack || true

# Build the ONSOUR Core into WASM
build-core:
	@echo "Building ONSOUR Core (Rust -> WASM)..."
	cd backend/core && wasm-pack build --target web --out-dir ../../pkg/core-wasm

# Build the Runtime and Synaptic Hub
build-runtime:
	@echo "Building ONSOUR Runtime & Hub..."
	cargo build

# Run all tests (Rust)
test:
	@echo "Running ONSOUR Integrated Tests..."
	cargo test

# Clean build artifacts
clean:
	@echo "Cleaning artifacts..."
	rm -rf target
	rm -rf pkg
