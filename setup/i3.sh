#!/bin/bash

set -e

echo "Starting i3 setup..."

if [ "$1" != "update" ]
then
    echo "Installing i3.."
    sudo pacman -S i3-gaps
fi

if [ "$1" != "update" ]
then
    echo "Installing tools used by i3..."
    sudo pacman -S alacritty screenfetch
    paru --skipreview --removemake --cleanafter -S sway-launcher-desktop
fi

echo "Installing i3 config..."
cp -r config/i3 ~/.config/

if [ "$1" != "update" ]
then
    echo "Adding i3 to xinitrc..."
    echo "i3" >> ~/.xinitrc
fi

if [ "$1" == "update" ]
then
    echo "Reloading i3..."
    i3-msg restart || true
fi

echo "Done with i3 setup!"
