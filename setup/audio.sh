#!/bin/bash

set -e

echo "Starting audio setup..."

if [ $1 != "update" ]
then
    echo "Installing pulseaudio..."
    sudo pacman -S pulseaudio pavucontrol
fi

echo "Installing Fiio Q3 udev rule..."
sudo cp extra/fiio_q3.rules /etc/udev/rules.d/

echo "Reloading udev rules..."
sudo udevadm control --reload

echo "Done with audio setup!"
