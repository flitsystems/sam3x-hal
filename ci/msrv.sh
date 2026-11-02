#!/usr/bin/env bash
# The published crates build with the oldest Rust their manifests promise.
# Cargo.lock is resolved for that version for the check, then put back, so
# the committed one keeps the newest dependencies.
set -euo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")/.."

msrv=$(sed -n 's/^rust-version = "\(.*\)"/\1/p' Cargo.toml)
rustup toolchain install "$msrv" --profile minimal --target thumbv7m-none-eabi

cp Cargo.lock Cargo.lock.orig
trap 'mv Cargo.lock.orig Cargo.lock' EXIT
CARGO_RESOLVER_INCOMPATIBLE_RUST_VERSIONS=fallback cargo generate-lockfile

cargo "+$msrv" check -p atsam3x8e-pac -p sam3x-hal --all-features
