# gossip-protocol-rs

Rust implementation of a synchronous gossip protocol.

## Build Rust library

Remove `crate-type = ["cdylib"]` from `Cargo.toml` to build as a static library.

```bash
git clone https://github.com/dineshpinto/gossip-protocol-rs.git
cd gossip-protocol-rs
cargo build --release
cargo run --release
```

## Build Python bindings

```bash
cd gossip-protocol-rs
poetry install
poetry run maturin develop
```

```ipython
>>> from gossip_protocol_rs import run_gossip_protocol
>>> res = run_gossip_protocol(num_honest_sample=5, num_adversarial_sample=4,
    num_non_sample=1000, num_peers=6, cycles=200)
```
