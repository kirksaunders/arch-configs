#!/bin/bash

set -e

echo "Starting wallpaper setup..."

echo "Installing feh..."
sudo pacman -S feh

echo "Downloading wallpaper..."
wget -O ~/.config/i3/wallpaper.png "https://initiate.alphacoders.com/download/wallpaper/959309/images/png/736299273287448"

echo "Adding feh command to .xprofile..."
echo "feh --bg-fill ~/.config/i3/wallpaper.png" >> ~/.xprofile

echo "Done with wallpaper setup..."