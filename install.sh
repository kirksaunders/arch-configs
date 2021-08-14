#!/bin/bash

echo "Beginning installation and setup..."
mkdir -f ~/.config
mkdir -f ~/bin

# Install the various components
./setup/audio.sh
./setup/xorg.sh
./setup/monitors.sh
./setup/mouse.sh
./setup/keyboard.sh
./setup/i3.sh
./setup/picom.sh
./setup/fonts.sh
./setup/theme.sh
./setup/wallpaper.sh
./setup/polybar.sh
./setup/tools.sh
./setup/spotify.sh
./setup/polkit.sh

echo "Installation complete!"