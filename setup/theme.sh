#!/bin/bash

set -e

echo "Starting theme setup..."

echo "Installing GTK 3..."
sudo pacman -S gtk3

echo "Downloading Juno GTK theme..."
mkdir -p ~/.themes
git clone https://github.com/EliverLara/Juno.git ~/.themes/Juno
rm -rf ~/.themes/Juno/.git

echo "Installing GTK config..."
cp -r config/gtk-3.0 ~/.config/

echo "Done with theme setup!"
