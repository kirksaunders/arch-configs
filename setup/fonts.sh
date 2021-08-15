#!/bin/bash

set -e

echo "Starting font setup..."

if [ $1 != "update" ]
then
    echo "Installing Fira Code and Nerd Fonts Complete..."
    sudo pacman -S ttf-fira-code ttf-nerd-fonts-symbols
fi

echo "Installing Material Design Icons..."
mkdir -p ~/.local/share/fonts
wget -O ~/.local/share/fonts/MaterialDesignIconsDesktop.ttf "https://github.com/Templarian/MaterialDesign-Font/blob/master/MaterialDesignIconsDesktop.ttf?raw=true"

fc-cache

echo "Done with font setup!"
