#!/bin/bash

set -e

echo "Starting polybar setup..."

if [ $1 != "update" ]
then
    echo "Installing polybar..."
    paru --skipreview --removemake --cleanafter -S polybar

    echo "Installing tools used by polybar..."
    sudo pacman -S gsimplecal pamixer lm_sensors bc playerctl pacman-contrib
fi

echo "Installing polybar config..."
cp -r config/polybar ~/.config/

echo "Done with polybar setup!"
