#!/bin/bash

set -e

echo "Starting nano setup..."

echo "Installing nano..."
sudo pacman -S nano

echo "Installing nano config..."
cp -r config/nano ~/.config/

echo "Done with nano setup!"
