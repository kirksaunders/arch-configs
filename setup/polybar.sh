#!/bin/bash

set -e

echo "Starting polybar setup..."

if [ "$1" != "update" ]
then
    echo "Installing polybar..."
    paru --skipreview --removemake --cleanafter -S polybar

    echo "Installing tools used by polybar..."
    sudo pacman -S gsimplecal pamixer lm_sensors bc playerctl pacman-contrib
    paru --skipreview --removemake --cleanafter -S checkupdates-aur
fi

echo "Installing polybar config..."
cp -r config/polybar ~/.config/

if [ "$1" == "update" ]
then
    echo "Restarting polybar..."
    killall polybar || true
    sleep .1
    i3-msg 'exec --no-startup-id ~/.config/polybar/scripts/launch.sh' || true
fi

echo "Done with polybar setup!"
