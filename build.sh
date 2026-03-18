#!/bin/bash
set -e
echo "Building console (aarch64-unknown-linux-musl)..."
cargo build --release --target aarch64-unknown-linux-musl
cp target/aarch64-unknown-linux-musl/release/console .
echo "Building container image..."
podman build --platform linux/arm64 -t registry.gt.lo:5000/console:edge .
rm -f console
echo "Done."
