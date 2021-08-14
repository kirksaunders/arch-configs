#!/bin/bash

set -e

echo "Starting polkit setup..."

echo "Installing polkit and mate-polkit..."
sudo pacman -S polkit mate-polkit

echo "Done with polkit setup!"