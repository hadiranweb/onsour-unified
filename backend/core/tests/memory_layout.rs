use core_engine::{Node, NodePractical, Edge};
use std::mem;

#[test]
fn test_memory_layout_v1_1() {
    let size_node = mem::size_of::<Node>();
    let size_node_p = mem::size_of::<NodePractical>();
    let size_edge = mem::size_of::<Edge>();

    println!("NodeMinimal size: {} bytes", size_node);
    println!("NodePractical size: {} bytes", size_node_p);
    println!("Edge size: {} bytes", size_edge);

    assert_eq!(size_node, 16);
    assert_eq!(size_node_p, 32);
    assert_eq!(size_edge, 16);
}
