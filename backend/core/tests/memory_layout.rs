use rts_core::state::{Node, NodePractical, Edge};
use std::mem;

#[test]
fn test_memory_layout() {
    let size_node = mem::size_of::<Node>();
    let size_node_p = mem::size_of::<NodePractical>();
    let size_edge = mem::size_of::<Edge>();

    println!("NodeMinimal size: {} bytes", size_node);
    println!("NodePractical size: {} bytes", size_node_p);
    println!("Edge size: {} bytes", size_edge);

    // Targets from spec:
    // NodeMinimal <= 16 bytes
    // NodePractical <= 32 bytes
    // Edge <= 16 bytes
    
    assert!(size_node <= 16, "NodeMinimal size {} exceeds target 16", size_node);
    assert!(size_node_p <= 32, "NodePractical size {} exceeds target 32", size_node_p);
    assert!(size_edge <= 16, "Edge size {} exceeds target 16", size_edge);
    
    // Write report
    let report = format!(
        "struct_name,size_bytes,alignment_bytes,target_bytes,pass\n\
        NodeMinimal,{},{},16,{}\n\
        NodePractical,{},{},32,{}\n\
        Edge,{},{},16,{}\n",
        size_node, mem::align_of::<Node>(), size_node <= 16,
        size_node_p, mem::align_of::<NodePractical>(), size_node_p <= 32,
        size_edge, mem::align_of::<Edge>(), size_edge <= 16
    );
    
    std::fs::write("../results/memory_report.csv", report).unwrap();
}
