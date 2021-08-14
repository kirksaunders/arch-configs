#!/bin/bash

echo "Starting i3 setup..."

echo "Installing i3.."
sudo pacman -S i3-gaps

echo "Installing tools used by i3..."
sudo pacman -S alacritty screenfetch
yay -S sway-launcher-desktop

echo "Installing i3 config..."
cp -r config/i3 ~/.config/

echo "Adding i3 to xinitrc..."
echo "i3" >> ~/.xinitrc

echo "Done with i3 setup!"
