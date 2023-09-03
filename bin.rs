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
    let _ = gossip_protocol_rs::gossip_protocol(
        num_honest_sample,
        num_adversarial_sample,
        num_non_sample,
        num_peers,
        cycles,
    );
}