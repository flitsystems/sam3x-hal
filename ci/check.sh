#!/usr/bin/env bash
# Everything CI checks, runnable locally with the same command. The workflow
# only calls this.
set -euo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")/.."

# rustup reads rust-toolchain.toml; a no-op once the toolchain is cached.
rustup toolchain install

cargo fmt --all --check
cargo clippy --workspace --all-features -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --all-features
