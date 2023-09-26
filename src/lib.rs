use pyo3::prelude::*;

use node::create_nodes;
use state_transition_function::evolve_state;

pub mod node;
pub mod state_transition_function;

pub fn run_gossip_protocol(
    num_honest_sample: usize,
    num_adversarial_sample: usize,
    num_non_sample: usize,
    num_peers: usize,
    cycles: usize,
    print_state: bool,
) -> Vec<i32> {

    // Create nodes
    if print_state {
        println!("Creating network (num_non_sample={}, num_honest_sample={}, num_adversarial_sample={}, \
        num_peers={})...", num_non_sample, num_honest_sample, num_adversarial_sample, num_peers
        );
    }

    let mut nodes = create_nodes(
        num_honest_sample,
        num_adversarial_sample,
        num_non_sample,
    );

    // Evolve state of the network
    if print_state {
        println!("Connecting nodes to random peers...");
    }

    evolve_state(&mut nodes, cycles, num_peers, print_state)
}

#[pyfunction]
pub fn pyrun_gossip_protocol(
    num_honest_sample: usize,
    num_adversarial_sample: usize,
    num_non_sample: usize,
    num_peers: usize,
    cycles: usize,
) -> Vec<i32> {
    run_gossip_protocol(
        num_honest_sample,
        num_adversarial_sample,
        num_non_sample,
        num_peers,
        cycles,
        false,
    )
}

#[pymodule]
fn gossip_protocol_rs(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(pyrun_gossip_protocol, m)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gossip_protocol() {
        let states = run_gossip_protocol(6, 4, 100, 6, 50, true);
        assert_eq!(states.len(), 100 * 50)
    }
}