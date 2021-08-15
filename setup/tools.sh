#!/bin/bash

set -e

echo "Starting tool setup..."

echo "Building ipc..."
cargo build --release --manifest-path=rust/ipc/Cargo.toml

if [ "$1" == "update" ]
then
    echo "Killing polybar..."
    killall polybar || true
    sleep .1
    killall ~/bin/ipc || true
    sleep .1
fi

echo "Installing ipc..."
cp rust/ipc/target/release/ipc ~/bin/

if [ "$1" == "update" ]
then
    echo "Starting polybar..."
    i3-msg 'exec --no-startup-id ~/.config/polybar/scripts/launch.sh' || true
fi

if [[ ( "$1" == "clean-tools" ) || ( $2 == "clean-tools" ) ]]
then
    echo "Cleaning up ipc..."
    cargo clean --manifest-path=rust/ipc/Cargo.toml
fi

sleep .25

echo "Building schedule..."
cargo build --release --manifest-path=rust/schedule/Cargo.toml

if [ "$1" == "update" ]
then
    echo "Killing polybar..."
    killall polybar || true
    sleep .1
    killall ~/bin/schedule || true
    sleep .1
fi

echo "Installing schedule..."
cp rust/schedule/target/release/schedule ~/bin/

if [ "$1" == "update" ]
then
    echo "Starting polybar..."
    i3-msg 'exec --no-startup-id ~/.config/polybar/scripts/launch.sh' || true
fi

if [[ ( "$1" == "clean-tools" ) || ( $2 == "clean-tools" ) ]]
then
    echo "Cleaning up schedule..."
    cargo clean --manifest-path=rust/schedule/Cargo.toml
fi

sleep .25

echo "Building text-animator..."
cargo build --release --manifest-path=rust/text-animator/Cargo.toml

if [ "$1" == "update" ]
then
    echo "Killing polybar..."
    killall polybar || true
    sleep .1
    killall ~/bin/text-animator || true
    sleep .1
fi

echo "Installing text-animator..."
cp rust/text-animator/target/release/text-animator ~/bin/

if [ "$1" == "update" ]
then
    echo "Starting polybar..."
    i3-msg 'exec --no-startup-id ~/.config/polybar/scripts/launch.sh' || true
fi

if [[ ( "$1" == "clean-tools" ) || ( $2 == "clean-tools" ) ]]
then
    echo "Cleaning up text-animator..."
    cargo clean --manifest-path=rust/text-animator/Cargo.toml
fi

echo "Done with tool setup!"
