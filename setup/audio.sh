#!/bin/bash

set -e

echo "Starting audio setup..."

echo "Installing pulseaudio..."
sudo pacman -S pulseaudio pavucontrol

echo "Done with audio setup!"