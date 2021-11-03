#!/bin/bash

set -e

echo "Starting xrdp setup..."

if [ "$1" == "update" ]
then
    echo "Nothing to do"
    exit 0
fi

echo "Installing xrdp..."
paru --skipreview --removemake --cleanafter -S xorgxrdp-nvidia

echo "Enabling xrdp service..."
sudo systemctl enable xrdp

echo "Done with xrdp setup!"
