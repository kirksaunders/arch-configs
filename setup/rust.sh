#!/bin/bash

set -e

echo "Starting rust setup..."

echo "Installing rust..."
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

echo "Done with rust setup!"
