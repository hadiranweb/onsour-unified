use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId, black_box};
use rts_core::{Node, Edge, step_sparse, step_node};
use std::time::Duration;

fn bench_rts_core_v04(c: &mut Criterion) {
    // 1. core_scalar_latency_ns
    c.bench_function("core_scalar_latency", |b| {
        let mut node = Node { theta: 0.1, e: 12.0, ec: 10.0 };
        b.iter(|| step_node(black_box(&mut node), black_box(0.5)))
    });

    // 2. sparse_graph_node_latency_ns (Official Track B)
    let mut group = c.benchmark_group("sparse_graph_node_latency");
    group.measurement_time(Duration::from_secs(5));

    let n_values = [1, 10, 100, 1000, 10000];
    let edges_per_node = 4;

    for n in n_values {
        let mut nodes = vec![Node { theta: 0.1, e: 12.0, ec: 10.0 }; n];
        let mut edges = Vec::new();
        for i in 0..n {
            for j in 1..=edges_per_node {
                edges.push(Edge {
                    src: i as u32,
                    dst: ((i + j) % n) as u32,
                    weight: 0.1,
                });
            }
        }

        group.bench_with_input(BenchmarkId::new("step_sparse", n), &n, |b, _| {
            b.iter(|| step_sparse(black_box(&mut nodes), black_box(&edges)))
        });
    }
    group.finish();
}

criterion_group!(benches, bench_rts_core_v04);
criterion_main!(benches);
