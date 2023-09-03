use node::{connect_nodes_to_peers, create_nodes};
use state_transition_function::evolve_state;

mod node;
mod state_transition_function;


fn main() {
    // Parameters
    let num_honest_sample = 5;
    let num_adversarial_sample = 4;
    let num_non_sample = 100;
    let time_steps = 20;

    // Create nodes
    let mut nodes = create_nodes(
        num_honest_sample,
        num_adversarial_sample,
        num_non_sample,
    );

    // Connect all nodes together
    connect_nodes_to_peers(&mut nodes);

    // Run gossip protocol simulation
    evolve_state(&mut nodes, time_steps);
}
