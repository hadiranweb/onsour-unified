use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

/// Minimal node structure (16 bytes aligned)
#[wasm_bindgen]
#[repr(C, align(16))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub struct Node {
    pub theta: f32,
    pub e: f32,
    pub ec: f32,
    pub _padding: u32, // Explicit pad to 16 bytes
}

/// Practical node structure (32 bytes aligned)
/// ARCHITECTURAL NOTE: 32 bytes is a divisor of 64 (standard cache line size).
/// wasm-bindgen doesn't support byte arrays, so we use scalar padding fields.
#[wasm_bindgen]
#[repr(C, align(32))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub struct NodePractical {
    pub theta: f32,
    pub theta_prev: f32,
    pub e: f32,
    pub ec: f32,
    pub alpha: f32,
    pub flags: u32,
    pub _pad1: u32, // 24 + 4 = 28 bytes
    pub _pad2: u32, // 28 + 4 = 32 bytes
}

#[wasm_bindgen]
#[repr(C, align(16))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub struct Edge {
    pub src: u32,
    pub dst: u32,
    pub weight: f32,
    pub _padding: u32, // Pad to 16 bytes
}

#[wasm_bindgen]
#[repr(C)]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub struct StochasticState {
    pub x: f64,
    pub r: f64,
    pub d: f64,
}

impl StochasticState {
    pub fn new(r: f64, d: f64) -> Self {
        Self { x: 0.0, r, d }
    }
    
    pub fn step(&mut self, dt: f64, eta: f64) {
        let force = 2.0 * self.r * self.x - 4.0 * self.x.powi(3);
        let stochastic = (2.0 * self.d * dt).sqrt() * eta;
        self.x += force * dt + stochastic;
    }
    
    pub fn omega(&self) -> f64 {
        2.0 * self.r.sqrt()
    }
}

// Compile-time size assertions
const _: () = assert!(std::mem::size_of::<Node>() == 16);
const _: () = assert!(std::mem::size_of::<NodePractical>() == 32);
const _: () = assert!(std::mem::size_of::<Edge>() == 16);
const _: () = assert!(std::mem::align_of::<NodePractical>() == 32);
