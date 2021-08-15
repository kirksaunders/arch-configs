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

if [ $1 == "update" ]
then
    echo "Restarting polybar..."
    killall polybar || true
    i3msg exec --no-startup-id ~/.config/polybar/scripts/launch.sh || true
fi

echo "Done with polybar setup!"
