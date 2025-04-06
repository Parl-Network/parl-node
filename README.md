# ZYTEX
All rights reserved to Xelis.

Zytex is the first fork of Xelis : A blockchain made in Rust and powered by Tokio, using account model.

To learn more, join our discord : https://discord.gg/zytex

## Config

### Network

- Expected Block Time is `15` seconds
- Address prefix is `prl`
- Transaction fee is `0.01000` ZTX per KB
- Up to `8` decimals
- Maximum supply: `18.4` millions
- Maximum block size: `1.25` MB
- Difficulty adjustment algorithm: retarget at every block
- Block reward emission: retarget at every block (Smooth decrease)

### Daemon

- Default P2P port is `5801`
- Defaut RPC Server port is `5800`

### Wallet

- Default RPC Server port is `8081`


## How to build

Building this project requires a working [Rust](https://rustup.rs) (stable) toolchain.

It's expected to be cross-platform and guaranteed to work on Linux, Windows, MacOS platforms.

### Build from sub project
Go to one of following folder you want to build from source: `parl_daemon`, `parl_miner` or `parl_wallet`.
To build a release (optimized) version:
`cargo build --release`

### Build from workspace
To build a version from workspace (parent folder) directly, use the option `--bin` with `parl_daemon`, `parl_miner` or `parl_wallet` as value.
Example: `cargo build --release --bin parl_miner`

You can also build a debug version (just remove `--release` option) or run it directly from cargo:
`cargo run`

## Funding

Current dev fee curve is as following:

- 10% from block 0 to 3 000 000 (expected time is one year with side blocks from blockDAG and network growing)
- 5% from 3 000 001 until the project being developed and stable enough to reduce it.
# parl-node
# parl-node
# parl-node
# parl-node
# parl-node
