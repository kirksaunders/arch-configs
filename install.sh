#!/bin/bash

set -e

if [ "$1" == "update" ]
then
    echo "Beginning update..."

    echo "Updating pacman packages..."
    sudo pacman -Syu

    echo "Updating paru packages..."
    paru --skipreview --removemake --cleanafter -Syu
else
    echo "Beginning installation and setup..."
fi

echo "Creating required user directories..."
mkdir -p ~/.config
mkdir -p ~/bin

# Install the various components
./setup/nano.sh $@
./setup/build.sh $@

./setup/rust.sh $@
echo "Sourcing rust environment variables..."
source ~/.cargo/env

./setup/paru.sh $@
./setup/audio.sh $@
./setup/xorg.sh $@
./setup/monitors.sh $@
./setup/mouse.sh $@
./setup/keyboard.sh $@
./setup/i3.sh $@
./setup/picom.sh $@
./setup/fonts.sh $@
./setup/theme.sh $@
./setup/wallpaper.sh $@
./setup/polybar.sh $@
./setup/tools.sh $@
./setup/spotify.sh $@
./setup/polkit.sh $@

if [ "$1" == "update" ]
then
    echo "Update complete!"
else
    echo "Installation complete!"
fi
