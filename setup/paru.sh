#!/bin/bash

set -e

echo "Starting paru setup..."

if [ "$1" == "update" ]
then
    echo "Nothing to do"
    exit 0
fi

echo "Installing paru..."
git clone https://aur.archlinux.org/paru.git
cd paru
makepkg -si

echo "Cleaning up paru..."
cd ..
rm -rf paru

echo "Adding paru skipreview alias to bashrc..."
echo 'alias paru='\''paru --skipreview'\' >> ~/.bashrc

echo "Done with paru setup!"
