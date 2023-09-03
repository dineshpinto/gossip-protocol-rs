use node::{connect_nodes_to_random_peers, create_nodes};
use state_transition_function::evolve_state;

mod node;
mod state_transition_function;

fn run_gossip_protocol(
    num_honest_sample: usize,
    num_adversarial_sample: usize,
    num_non_sample: usize,
    num_peers: usize,
    cycles: usize,
) -> Vec<f32> {
    // Create nodes
    let mut nodes = create_nodes(
        num_honest_sample,
        num_adversarial_sample,
        num_non_sample,
    );

    // Connect all nodes together
    connect_nodes_to_random_peers(&mut nodes, num_peers);

    // Run gossip protocol simulation
    evolve_state(&mut nodes, cycles)
}

fn main() {
    // Parameters
    // Number of honest nodes in sample
    let num_honest_sample = 5;
    // Number of adversarial nodes in sample
    let num_adversarial_sample = 4;
    // Number of non-sample nodes
    let num_non_sample = 1000;
    // Number of peers each node connects to
    let num_peers = 4;
    // Number of cycles to run the simulation
    let cycles = 200;

    // Run gossip protocol simulation
    let _ = run_gossip_protocol(
        num_honest_sample,
        num_adversarial_sample,
        num_non_sample,
        num_peers,
        cycles,
    );
}
