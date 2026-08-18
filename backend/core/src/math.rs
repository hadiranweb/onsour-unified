use crate::state::Node;

pub fn step_node_math(theta_prev: f32, e: f32, ec: f32, neighbor_sum: f32) -> f32 {
    let raw = if ec == 0.0 {
        neighbor_sum
    } else {
        (e - ec) / ec + neighbor_sum
    };
    
    raw.tanh()
}

pub fn step_node(node: &mut Node, neighbor_sum: f32) -> f32 {
    node.theta = step_node_math(node.theta, node.e, node.ec, neighbor_sum);
    node.theta
}

pub fn alpha(theta: f32) -> f32 {
    (theta + 1.0) / 2.0
}
