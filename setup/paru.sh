#!/bin/bash

echo "Starting paru setup..."

echo "Installing paru..."
git clone https://aur.archlinux.org/paru.git
cd paru
makepkg -si

echo "Cleaning up paru..."
cd ..
rm -rf paru

echo "Adding paru skipreview alias to bashrc..."
echo 'alias paru="paru --skipreview"' >> ~/.bashrc

echo "Done with paru setup!"