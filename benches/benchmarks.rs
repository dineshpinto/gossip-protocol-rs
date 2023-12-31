use criterion::{black_box, Criterion, criterion_group, criterion_main};

use gossip_protocol_rs::run_gossip_protocol;
use gossip_protocol_rs::node::{connect_nodes_to_random_peers, create_nodes};
use gossip_protocol_rs::state_transition_function::evolve_state;

fn criterion_benchmark(c: &mut Criterion) {
    let num_honest_sample = 6;
    let num_adversarial_sample = 4;
    let num_non_sample = 1000;
    let num_peers = 6;
    let cycles = 200;

    // Create nodes benchmark
    c.bench_function(
        "create_nodes",
        |b| b.iter(||
            create_nodes(
                black_box(num_honest_sample),
                black_box(num_adversarial_sample),
                black_box(num_non_sample),
            )
        ),
    );

    // Connect nodes benchmark
    let mut nodes = create_nodes(
        num_honest_sample,
        num_adversarial_sample,
        num_non_sample,
    );

    c.bench_function(
        "connect_nodes_to_random_peers",
        |b| b.iter(||
            connect_nodes_to_random_peers(
                black_box(&mut nodes),
                black_box(num_peers),
            )
        ),
    );

    // Evolve state benchmark
    let mut nodes = create_nodes(
        num_honest_sample,
        num_adversarial_sample,
        num_non_sample,
    );
    connect_nodes_to_random_peers(&mut nodes, num_peers);

    c.bench_function(
        "evolve_state",
        |b| b.iter(||
            evolve_state(
                black_box(&mut nodes),
                black_box(cycles),
                black_box(num_peers),
                black_box(false),
            )
        ),
    );

    c.bench_function(
        "run_gossip_protocol",
        |b| b.iter(||
            run_gossip_protocol(
                black_box(num_honest_sample),
                black_box(num_adversarial_sample),
                black_box(num_non_sample),
                black_box(num_peers),
                black_box(cycles),
                black_box(false),
            )
        ),
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);