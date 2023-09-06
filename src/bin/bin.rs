fn main() {
    // Parameters
    // Number of honest nodes in sample
    let num_honest_sample = 300;
    // Number of adversarial nodes in sample
    let num_adversarial_sample = 200;
    // Number of non-sample nodes
    let num_non_sample = 10000;
    // Number of peers each node connects to
    let num_peers = 8;
    // Number of cycles to run the simulation
    let cycles = 50;

    // Run gossip protocol simulation
    let _ = gossip_protocol_rs::run_gossip_protocol(
        num_honest_sample,
        num_adversarial_sample,
        num_non_sample,
        num_peers,
        cycles,
        true,
    );
    println!("Done!");
}