#!/bin/bash

echo "Starting polybar setup..."

echo "Installing polybar..."
yay -S polybar

echo "Installing tools used by polybar..."
sudo pacman -S gsimplecal pamixer lm_sensors bc playerctl pacman-contrib
yay -S sway-launcher-desktop

echo "Installing polybar config..."
cp -r config/i3 ~/.config/

echo "Done with polybar setup!"
