use std::cmp::Ordering;
use std::collections::HashMap;

use rand::seq::SliceRandom;

#[derive(PartialEq, Debug, Clone)]
pub(crate) enum Message {
    Honest,
    Adversarial,
    Default,
}

#[derive(PartialEq, Debug, Clone)]
pub(crate) struct Node {
    node_id: usize,
    messages: Vec<Message>,
    pub(crate) initial_message: Message,
    pub(crate) peers: Vec<usize>,
}

impl Node {
    pub(crate) fn new(node_id: usize, initial_message: Message) -> Node {
        Node {
            node_id,
            messages: Vec::new(),
            initial_message,
            peers: Vec::new(),
        }
    }

    pub(crate) fn add_peers(&mut self, peer_list: Vec<usize>) {
        self.peers = peer_list;
    }

    pub(crate) fn update(&mut self, messages: &mut Vec<Message>) {
        if self.initial_message != Message::Default {} else {
            self.messages.append(messages);
        }
    }

    pub(crate) fn broadcast(&self) -> Message {
        return if self.initial_message != Message::Default {
            self.initial_message.clone()
        } else {
            let honest_count = self.messages
                .iter()
                .filter(|&n| *n == Message::Honest)
                .count();
            let adversarial_count = self.messages
                .iter()
                .filter(|&n| *n == Message::Adversarial)
                .count();

            match honest_count.cmp(&adversarial_count) {
                Ordering::Greater => Message::Honest,
                Ordering::Less => Message::Adversarial,
                Ordering::Equal => Message::Default,
            }
        };
    }
}


pub(crate) fn create_nodes(
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

pub(crate) fn connect_nodes_to_random_peers(
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
