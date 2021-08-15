#!/bin/bash

set -e

echo "Starting nano setup..."

if [ "$1" != "update" ]
then
    echo "Installing nano..."
    sudo pacman -S nano
fi

echo "Installing nano config..."
cp -r config/nano ~/.config/

echo "Done with nano setup!"
