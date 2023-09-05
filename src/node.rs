use std::collections::HashMap;

use counter::Counter;
use rand::seq::SliceRandom;

/// A message that can be broadcast by a node
#[derive(PartialEq, Debug, Clone, Eq, Hash, Ord, PartialOrd, Copy)]
pub enum Message {
    Honest = 1,
    Adversarial = 0,
    Default = -1,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Node {
    node_id: usize,
    msg_counter: Counter<Message>,
    pub(crate) initial_message: Message,
    pub(crate) peers: Vec<usize>,
}

impl Node {
    /// Returns a node with the given node_id and initial message
    ///
    /// # Arguments
    ///
    /// * `node_id` - A unique integer identifier for the node
    /// * `initial_message` - The initial message the node will broadcast
    pub fn new(node_id: usize, initial_message: Message) -> Node {
        Node {
            node_id,
            msg_counter: Counter::new(),
            initial_message,
            peers: Vec::new(),
        }
    }

    /// Adds a list of peers to the node
    /// # Arguments
    /// * `peer_list` - A list of `node_ids corresponding to the peers of the node
    pub fn add_peers(&mut self, peer_list: Vec<usize>) {
        self.peers = peer_list;
    }

    /// Updates the node with a list of messages. The messages are appended to the list of messages
    /// the node has received.
    /// # Arguments
    /// * `messages` - A list of messages to update the node with
    pub fn update(&mut self, messages: Vec<Message>) {
        if self.initial_message != Message::Default {} else {
            self.msg_counter.update(messages);
        }
    }

    /// Returns the message the node will broadcast. If the node has an initial message, it will
    /// return that message. Otherwise, it will return the majority message from the messages it
    /// has received from its peers.
    pub fn broadcast(&self) -> Message {
        return if self.initial_message != Message::Default {
            self.initial_message
        } else {
            self.msg_counter
                .most_common_ordered()
                .first()
                .unwrap_or(&(self.initial_message, 0))
                .0
        };
    }
}


/// Creates a list of nodes with the given number of honest, adversarial and non-sample nodes
/// # Arguments
/// * `num_honest_sample` - The number of honest nodes in the sample
/// * `num_adversarial_sample` - The number of adversarial nodes in the sample
/// * `num_non_sample` - The number of non-sample nodes
pub fn create_nodes(
    num_honest_sample: usize,
    num_adversarial_sample: usize,
    num_non_sample: usize,
) -> HashMap<usize, Node> {
    let total_nodes = num_honest_sample + num_adversarial_sample + num_non_sample;

    let mut nodes: HashMap<usize, Node> = HashMap::new();
    for i in 0..total_nodes {
        if i < num_honest_sample {
            nodes.insert(i, Node::new(i, Message::Honest));
        } else if i < num_honest_sample + num_adversarial_sample {
            nodes.insert(i, Node::new(i, Message::Adversarial));
        } else {
            nodes.insert(i, Node::new(i, Message::Default));
        }
    }
    nodes
}

/// Connects each node to a random set of peers
/// # Arguments
/// * `nodes` - A list of nodes to connect
/// * `num_peers` - The number of peers each node connects to
pub fn connect_nodes_to_random_peers(
    nodes: &mut HashMap<usize, Node>,
    num_peers: usize,
) {
    let mut rng = &mut rand::thread_rng();

    let total_nodes = nodes.len();
    for (node_id, node) in nodes {
        let mut _peers = (0..total_nodes).collect::<Vec<usize>>();
        _peers.remove(*node_id);
        let peers: Vec<usize> = _peers
            .choose_multiple(&mut rng, num_peers)
            .cloned()
            .collect();
        node.add_peers(peers);
    }
}
