#!/bin/bash

echo "Starting keyboard setup..."

echo "Installing keyboard config..."
sudo cp extra/hid_apple.conf /etc/modprobe.d/

echo "Done with keyboard setup!"
