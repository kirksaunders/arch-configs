#!/bin/bash

echo "Starting tool setup..."

echo "Building ipc..."
cargo build --release --manifest-path=rust/ipc/Cargo.toml

echo "Installing ipc..."
cp rust/ipc/target/release/ipc ~/bin/

echo "Building schedule..."
cargo build --release --manifest-path=rust/schedule/Cargo.toml

echo "Installing schedule..."
cp rust/schedule/target/release/schedule ~/bin/

echo "Building text-animator..."
cargo build --release --manifest-path=rust/text-animator/Cargo.toml

echo "Installing text-animator..."
cp rust/text-animator/target/release/text-animator ~/bin/

echo "Done with tool setup!"
