#!/bin/bash

set -e

echo "Starting picom setup..."

echo "Installing picom..."
sudo pacman -S picom

echo "Installing picom config..."
cp -r config/picom ~/.config/

echo "Done with picom setup!"
