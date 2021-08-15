#!/bin/bash

set -e

echo "Starting build tools setup..."

echo "Installing build tools..."
sudo pacman -S base-devel git wget

echo "Done with build tools setup!"
