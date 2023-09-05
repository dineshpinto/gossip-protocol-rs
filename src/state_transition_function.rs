use std::collections::HashMap;

use crate::node::{Message, Node};

fn average(numbers: &[i32]) -> f32 {
    numbers.iter().sum::<i32>() as f32 / numbers.len() as f32
}

/// Evolves the state of the nodes by running the gossip protocol simulation
/// # Arguments
/// * `nodes` - A list of nodes
/// * `cycles` - The number of cycles to run the simulation
pub fn evolve_state(
    nodes: &mut HashMap<usize, Node>,
    cycles: usize,
    print_cycle: bool,
) -> Vec<i32> {
    let mut non_sample_broadcasts = Vec::new();

    let mut _message_queue: HashMap<usize, Vec<Message>> = HashMap::new();
    let mut _non_sample_broadcasts: Vec<i32> = Vec::new();

    // Run simulation
    for cycle in 0..cycles {
        // Create a message queue to store messages corresponding to each node
        // Record broadcast messages from non-sample nodes

        // Collect broadcast messages and store them in the message queue
        for node in &mut nodes.values() {
            let msg = node.broadcast();

            // Skip default messages
            if msg == Message::Default {
                continue;
            }

            for node_id in &node.peers {
                _message_queue
                    .entry(*node_id)
                    .or_insert(vec![])
                    .push( msg);
            }

            // Record the broadcast message from non-sample nodes
            if node.initial_message == Message::Default {
                match msg {
                    Message::Honest => { _non_sample_broadcasts.push(1) }
                    Message::Adversarial => { _non_sample_broadcasts.push(0) }
                    Message::Default => { _non_sample_broadcasts.push(-1) }
                }
            }
        }

        // Update nodes with messages from the message queue
        for (node_id, messages) in &_message_queue {
            nodes
                .get_mut(node_id)
                .unwrap()
                .update(messages.clone());
        }

        non_sample_broadcasts.append(&mut _non_sample_broadcasts);
        if print_cycle {
            println!("Cycle {}, Average value broadcast: {:?}",
                     cycle, average(&non_sample_broadcasts));
        }

        _message_queue.clear();
        _non_sample_broadcasts.clear();
    }
    non_sample_broadcasts
}