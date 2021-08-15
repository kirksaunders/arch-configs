#!/bin/bash

set -e

echo "Starting build tools setup..."

if [ "$1" == "update" ]
then
    echo "Nothing to do"
    exit 0
fi

echo "Installing build tools..."
sudo pacman -S base-devel git wget

echo "Done with build tools setup!"
