# Jambo

A Jambo Substrate node, ready for hacking.

## Building

Install Rust:

```bash
curl https://sh.rustup.rs -sSf | sh
```

NOTE: change install setting [default toolchain: stable (default)] -> nightly.

```bash
rustup default nightly
```

Clone Code:

```bash
git clone https://github.com/JAMBO-NETWORK/substrate-node.git
cd substrate-node
git submodule init
git submodule update
```

Install required rust target(wasm32):

```bash
.maintain/init.sh
```

Build Jambo node:

```bash
cargo build --release
```

Ensure you have a fresh start if updating from another version:

```bash
./target/release/substrate-node purge-chain
```

To start a Jambo node, run:

```bash
./target/release/substrate-node
```

To start the Jambo validater node, run:

```bash
./target/release/substrate-node \
  --name NodeName \
  --validator \
  --node-key ..... \
  --keystore-path /path/to/auth
```

To start a local dev node, run:

```bash
cargo run --release -- --dev --alice --tmp
```
