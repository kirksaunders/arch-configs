#!/bin/bash

echo "Starting theme setup..."

echo "Installing GTK 3..."
sudo pacman -S gtk3

echo "Downloading Juno GTK theme..."
mkdir -f ~/.themes
git clone git@github.com:EliverLara/Juno.git ~/.themes/

echo "Installing GTK config..."
cp -r config/gtk-3 ~/.config/

echo "Done with theme setup!"
