#[repr(C, align(32))]
#[derive(Debug, Clone, Copy, Default)]
pub struct Node {
    pub theta: f32,
    pub e: f32,
    pub ec: f32,
    pub _padding: [u8; 20],
}

#[repr(C, align(32))]
#[derive(Debug, Clone, Copy, Default)]
pub struct NodePractical {
    pub theta: f32,      // 4 bytes
    pub theta_prev: f32, // 4 bytes
    pub e: f32,          // 4 bytes
    pub ec: f32,         // 4 bytes
    pub alpha: f32,      // 4 bytes
    pub flags: u32,      // 4 bytes
    pub _padding: [u8; 8], // 8 bytes -> Total 32 bytes
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct Edge {
    pub src: u32,
    pub dst: u32,
    pub weight: f32,
}

#[repr(C, align(32))]
#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct StochasticState {
    pub x: f64,
    pub r: f64,
    pub d: f64,
    pub _padding: [u8; 8], // 24 + 8 = 32 bytes
}

impl StochasticState {
    pub fn new(r: f64, d: f64) -> Self {
        Self { x: 0.0, r, d, _padding: [0; 8] }
    }

    pub fn step(&mut self, dt: f64, eta: f64) {
        let force = 2.0 * self.r * self.x - 4.0 * self.x.powi(3);
        let stochastic = (2.0 * self.d * dt).sqrt() * eta;
        self.x += force * dt + stochastic;
    }
}
