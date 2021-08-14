#!/bin/bash

echo "Starting xorg setup..."

echo "Installing xorg server..."
sudo pacman -S xorg-server xorg-xinit xdotool

echo "Adding default mouse pos to xinitrc..."
echo "xdotool mousemove 960 540" >> ~/.xinitrc

echo "Done with xorg setup!"
