from gossip_protocol_rs import pyrun_gossip_protocol

if __name__ == "__main__":
    num_honest_sample = 6
    num_adversarial_sample = 4
    num_non_sample = 1000
    num_peers = 6
    cycles = 200

    res = pyrun_gossip_protocol(
        num_honest_sample=num_honest_sample,
        num_adversarial_sample=num_adversarial_sample,
        num_non_sample=num_non_sample,
        num_peers=num_peers,
        cycles=cycles
    )