#!/bin/bash

set -e

echo "Starting mouse setup..."

echo "Installing mouse acceleration config..."
sudo cp extra/50-mouse-acceleration.conf /etc/X11/xorg.conf.d/

echo "Done with mouse setup!"
