#!/bin/bash

set -e

echo "Starting rust setup..."

if [ "$1" == "update" ]
then
    echo "Nothing to do"
    exit 0
fi

echo "Installing rust..."
sudo pacman -S rustup
rustup default stable

echo "Done with rust setup!"
