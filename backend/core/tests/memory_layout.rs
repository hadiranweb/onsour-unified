use rts_core::state::{Node, NodePractical, Edge};
use std::mem;

#[test]
fn test_memory_layout_v1_1() {
    let size_node = mem::size_of::<Node>();
    let size_node_p = mem::size_of::<NodePractical>();
    let size_edge = mem::size_of::<Edge>();

    println!("NodeMinimal size: {} bytes", size_node);
    println!("NodePractical size: {} bytes", size_node_p);
    println!("Edge size: {} bytes", size_edge);

    // CRITICAL: Size must be power of 2 or divisor of 64 to avoid straddling
    assert_eq!(size_node, 16, "NodeMinimal size must be exactly 16 bytes");
    assert_eq!(size_node_p, 32, "NodePractical size must be exactly 32 bytes");
    assert_eq!(size_edge, 16, "Edge size must be exactly 16 bytes");
    
    // CRITICAL: Alignment must match size for perfect vectorization
    assert_eq!(mem::align_of::<NodePractical>(), 32, "NodePractical alignment must be 32 bytes");
    
    // Verify no straddling in Vec (Stride check)
    let nodes = vec![NodePractical::default(); 2];
    let first_addr = &nodes[0] as *const _ as usize;
    let second_addr = &nodes[1] as *const _ as usize;
    assert_eq!(second_addr - first_addr, 32, "Stride must be 32 bytes to ensure cache alignment");
}

#[test]
fn test_repr_c_offsets() {
    // Note: mem::offset_of! requires Rust 1.77+. 
    // We use a simple pointer math fallback for older versions if needed, 
    // but here we assume modern environment.
    
    let node = NodePractical::default();
    let base = &node as *const _ as usize;
    
    assert_eq!(&node.theta as *const _ as usize - base, 0);
    assert_eq!(&node.theta_prev as *const _ as usize - base, 4);
    assert_eq!(&node.e as *const _ as usize - base, 8);
    assert_eq!(&node.ec as *const _ as usize - base, 12);
    assert_eq!(&node.alpha as *const _ as usize - base, 16);
    assert_eq!(&node.flags as *const _ as usize - base, 20);
    assert_eq!(&node._pad1 as *const _ as usize - base, 24);
    assert_eq!(&node._pad2 as *const _ as usize - base, 28);
}
