#!/bin/bash

set -e

echo "Starting tool setup..."

echo "Building ipc..."
cargo build --release --manifest-path=rust/ipc/Cargo.toml

echo "Installing ipc..."
cp rust/ipc/target/release/ipc ~/bin/

if [ $1 == "clean-tools" ] || [ $2 == "clean-tools" ]
then
    echo "Cleaning up ipc..."
    cargo clean --manifest-path=rust/ipc/Cargo.toml
fi

echo "Building schedule..."
cargo build --release --manifest-path=rust/schedule/Cargo.toml

echo "Installing schedule..."
cp rust/schedule/target/release/schedule ~/bin/

if [ $1 == "clean-tools" ] || [ $2 == "clean-tools" ]
then
    echo "Cleaning up schedule..."
    cargo clean --manifest-path=rust/schedule/Cargo.toml
fi

echo "Building text-animator..."
cargo build --release --manifest-path=rust/text-animator/Cargo.toml

echo "Installing text-animator..."
cp rust/text-animator/target/release/text-animator ~/bin/

if [ $1 == "clean-tools" ] || [ $2 == "clean-tools" ]
then
    echo "Cleaning up text-animator..."
    cargo clean --manifest-path=rust/text-animator/Cargo.toml
fi

echo "Done with tool setup!"
