#!/bin/bash

set -e

echo "Beginning installation and setup..."

echo "Creating required user directories..."
mkdir -p ~/.config
mkdir -p ~/bin

# Install the various components
./setup/nano.sh
./setup/build.sh

./setup/rust.sh
echo "Sourcing rust environment variables..."
source ~/.cargo/env

./setup/paru.sh
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
