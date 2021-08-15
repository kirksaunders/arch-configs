#!/bin/bash

set -e

echo "Starting picom setup..."

if [ "$1" != "update" ]
then
    echo "Installing picom..."
    sudo pacman -S picom
fi

echo "Installing picom config..."
cp -r config/picom ~/.config/

echo "Done with picom setup!"
