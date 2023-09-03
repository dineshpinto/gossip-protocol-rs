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
) -> Vec<i32> {
    let mut non_sample_broadcasts = Vec::new();
    // Run simulation
    for cycle in 0..cycles {
        // Create a message queue to store messages corresponding to each node
        let mut _message_queue: HashMap<usize, Vec<Message>> = HashMap::new();
        // Record broadcast messages from non-sample nodes
        let mut _non_sample_broadcasts: Vec<i32> = Vec::new();

        // Collect broadcast messages and store them in the message queue
        for node in &mut nodes.values() {
            let msg = node.broadcast();

            for node_id in &node.peers {
                _message_queue
                    .entry(*node_id)
                    .or_insert(vec![])
                    .push(msg.clone());
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
        for (node_id, messages) in &mut _message_queue {
            nodes
                .get_mut(node_id)
                .unwrap()
                .update(messages);
        }

        non_sample_broadcasts.append(&mut _non_sample_broadcasts);
        // println!("Cycle {}, Average value broadcast: {:?}",
        //          cycle, non_sample_broadcasts.last().unwrap());
    }
    non_sample_broadcasts
}