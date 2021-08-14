#!/bin/bash

echo "Beginning installation and setup..."
mkdir -f ~/.config
mkdir -f ~/bin

echo "Adding bin to path..."
echo 'export PATH=$PATH:~/bin/;' >> ~/.xprofile

# Install the various components
./setup/audio.sh
./setup/xorg.sh
./setup/monitors.sh
./setup/mouse.sh
./setup/i3.sh
./setup/picom.sh
./setup/fonts.sh
./setup/theme.sh
./setup/wallpaper.sh
./setup/polybar.sh
./setup/tools.sh
./setup/spotify.sh

echo "Installation complete!"