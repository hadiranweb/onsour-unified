use rts_core::{Node, NodePractical, StochasticState};
use std::mem::{size_of, align_of};

#[test]
fn test_node_layouts() {
    // Verify Node
    assert_eq!(size_of::<Node>(), 32, "Node size must be 32 bytes");
    assert_eq!(align_of::<Node>(), 32, "Node alignment must be 32 bytes");

    // Verify NodePractical
    assert_eq!(size_of::<NodePractical>(), 32, "NodePractical size must be 32 bytes");
    assert_eq!(align_of::<NodePractical>(), 32, "NodePractical alignment must be 32 bytes");

    // Verify StochasticState
    assert_eq!(size_of::<StochasticState>(), 32, "StochasticState size must be 32 bytes");
    assert_eq!(align_of::<StochasticState>(), 32, "StochasticState alignment must be 32 bytes");

    // Verify Stride in Vec
    let nodes = vec![NodePractical::default(); 2];
    let addr0 = &nodes[0] as *const _ as usize;
    let addr1 = &nodes[1] as *const _ as usize;
    assert_eq!(addr1 - addr0, 32, "Stride between elements in Vec must be 32 bytes");
}
