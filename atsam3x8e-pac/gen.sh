#!/usr/bin/env bash
# Regenerate the PAC from the vendored SVD. The generated output IS committed:
# consumers never need svd2rust installed.

set -euo pipefail
cd "$(dirname "$0")"

svdtools patch svd/patches/atsam3x8e.yaml
# --reexport-interrupt: svd2rust stopped re-exporting cortex-m-rt's
# `#[interrupt]` by default in 0.33.
svd2rust --target cortex-m --reexport-interrupt -i svd/ATSAM3X8E.svd.patched

rm -rf src
form -i lib.rs -o src
rm lib.rs
cargo fmt
