#!/bin/bash

set -e

echo "Starting polybar setup..."

echo "Installing polybar..."
paru -S polybar

echo "Installing tools used by polybar..."
sudo pacman -S gsimplecal pamixer lm_sensors bc playerctl pacman-contrib

echo "Installing polybar config..."
cp -r config/i3 ~/.config/

echo "Done with polybar setup!"
