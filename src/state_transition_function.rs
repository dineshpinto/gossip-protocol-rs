use std::collections::HashMap;

use crate::node::{Message, Node};

fn average(numbers: &[i32]) -> f32 {
    numbers.iter().sum::<i32>() as f32 / numbers.len() as f32
}

/// Evolves the state of the nodes by running the gossip protocol simulation
/// # Arguments
/// * `nodes` - A list of nodes
/// * `cycles` - The number of cycles to run the simulation
/// # Example
/// ```
/// use node::{connect_nodes_to_random_peers, create_nodes};
/// use state_transition_function::evolve_state;
/// let mut nodes = create_nodes(5, 4, 1000);
/// connect_nodes_to_random_peers(&mut nodes, 6);
/// evolve_state(&mut nodes, 100);
/// ```
pub(crate) fn evolve_state(
    nodes: &mut HashMap<usize, Node>,
    cycles: usize,
) -> Vec<f32> {
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
                    Message::Default => {}
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

        println!("Cycle {}, Average value broadcast: {}",
                 cycle, average(&_non_sample_broadcasts));
        non_sample_broadcasts.push(average(&_non_sample_broadcasts) as f32);
    }
    non_sample_broadcasts
}