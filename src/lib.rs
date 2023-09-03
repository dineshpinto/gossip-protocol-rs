use pyo3::prelude::*;

use node::{connect_nodes_to_random_peers, create_nodes};
use state_transition_function::evolve_state;

mod node;
mod state_transition_function;

pub fn gossip_protocol(
    num_honest_sample: usize,
    num_adversarial_sample: usize,
    num_non_sample: usize,
    num_peers: usize,
    cycles: usize,
) -> Vec<i32> {
    // Create nodes
    let mut nodes = create_nodes(
        num_honest_sample,
        num_adversarial_sample,
        num_non_sample,
    );

    // Connect nodes together
    connect_nodes_to_random_peers(&mut nodes, num_peers);

    // Evolve state of the network
    evolve_state(&mut nodes, cycles)
}

#[pyfunction]
pub fn run_gossip_protocol(
    num_honest_sample: usize,
    num_adversarial_sample: usize,
    num_non_sample: usize,
    num_peers: usize,
    cycles: usize,
) -> Vec<i32> {
    // Create nodes
    let mut nodes = create_nodes(
        num_honest_sample,
        num_adversarial_sample,
        num_non_sample,
    );

    // Connect nodes together
    connect_nodes_to_random_peers(&mut nodes, num_peers);

    // Evolve state of the network
    evolve_state(&mut nodes, cycles)
}

#[pymodule]
fn gossip_protocol_rs(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(run_gossip_protocol, m)?)?;
    Ok(())
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use test::Bencher;
//
//     #[bench]
//     fn bench_create_nodes(b: &mut Bencher) {
//         b.iter(|| create_nodes(4, 4, 10000));
//     }
//
//     #[bench]
//     fn bench_connect_nodes(b: &mut Bencher) {
//         let mut nodes =  create_nodes(4, 4, 10000);
//         b.iter(|| connect_nodes_to_random_peers(&mut nodes, 6));
//     }
//
//     #[bench]
//     fn bench_evolve_nodes(b: &mut Bencher) {
//         let mut nodes =  create_nodes(4, 4, 10000);
//         connect_nodes_to_random_peers(&mut nodes, 6);
//         b.iter(|| evolve_state(&mut nodes, 10));
//     }
// }