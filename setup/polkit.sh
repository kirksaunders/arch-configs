#!/bin/bash

set -e

echo "Starting polkit setup..."

if [ "$1" == "update" ]
then
    echo "Nothing to do"
    exit 0
fi

echo "Installing polkit and mate-polkit..."
sudo pacman -S polkit mate-polkit

echo "Done with polkit setup!"
