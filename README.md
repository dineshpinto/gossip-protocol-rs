[![test](https://github.com/dineshpinto/gossip-protocol-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/dineshpinto/gossip-protocol-rs/actions/workflows/rust.yml)
[![codecov](https://codecov.io/gh/dineshpinto/gossip-protocol-rs/graph/badge.svg?token=DRI0SYP28V)](https://codecov.io/gh/dineshpinto/gossip-protocol-rs)
# gossip-protocol-rs

A blazingly-fast implementation of a synchronous gossip protocol in Rust with PyO3 Python bindings.

## Build Rust library

```bash
cargo build --release
cargo run --release
```

## Build Python bindings (optional, for data analytics)

Add `crate-type = ["cdylib"]` under `[lib]` to `Cargo.toml`, then:

```bash
poetry install --no-root
poetry run maturin build --release
poetry add target/wheels/*.whl
```

### Call the Rust functions from Python

```ipython
>>> from gossip_protocol_rs import pyrun_gossip_protocol
>>> res = pyrun_gossip_protocol(num_honest_sample=3000, num_adversarial_sample=2000,
    num_non_sample=100000, num_peers=8, cycles=50)
```

## Simulation

![Results](output/convergence_heatmap_and_overall.png)
![Results](output/node_state_heatmap.png)

See [dineshpinto/synchronous-gossip-protocol](https://github.com/dineshpinto/synchronous-gossip-protocol) for a pure
Python implementation, along with additional
theoretical details.

## Benchmark

Uses `criterion` to benchmark the library.

```bash
cargo bench
```

| Param           | Value |
|-----------------|-------|
| num_non_sample  | 1000  |
| num_honest      | 6     | 
| num_adversarial | 4     |
| num_peers       | 6     |
| cycles          | 200   |

![create_nodes_pdf](output/benchmark_create_nodes_pdf.svg)
![connect_nodes_to_random_peers_pdf](output/benchmark_connect_nodes_to_random_peers_pdf.svg)
![evolve_state_pdf](output/benchmark_evolve_state_pdf.svg)
