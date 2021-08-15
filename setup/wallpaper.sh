#!/bin/bash

set -e

echo "Starting wallpaper setup..."

if [ "$1" != "update" ]
then
    echo "Installing feh..."
    sudo pacman -S feh
fi

echo "Downloading wallpaper..."
wget -O ~/.config/i3/wallpaper.png "https://initiate.alphacoders.com/download/wallpaper/959309/images/png/736299273287448"

if [ "$1" != "update" ]
then
    echo "Adding feh command to .xprofile..."
    echo "feh --bg-fill ~/.config/i3/wallpaper.png" >> ~/.xprofile
fi

echo "Done with wallpaper setup..."
