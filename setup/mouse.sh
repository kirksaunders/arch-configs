#!/bin/bash

set -e

echo "Starting mouse setup..."

echo "Installing mouse sens script..."
sudo cp extra/mousesens.sh /usr/local/bin/

echo "Installing mouse sens udev rule..."
sudo cp extra/mousesens.rules /etc/udev/rules.d/

echo "Reloading udev rules..."
sudo udevadm control --reload

echo "Installing config to disable mouse acceleration..."
sudo cp extra/50-mouse-acceleration.conf /etc/X11/xorg.conf.d/

echo "Done with mouse setup!"