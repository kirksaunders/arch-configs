#!/bin/bash

set -e

echo "Starting font setup..."

echo "Installing Fira Code..."
sudo pacman -S ttf-fira-code

echo "Installing Material Design Icons..."
wget -O ~/.local/share/fonts/MaterialDesignIconsDesktop.ttf "https://github.com/Templarian/MaterialDesign-Font/blob/master/MaterialDesignIconsDesktop.ttf?raw=true"

fc-cache

echo "Done with font setup!"
