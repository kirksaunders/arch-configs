#!/bin/bash

set -e

echo "Starting monitor setup..."

echo "Installing monitor config..."
sudo cp extra/10-monitor.conf /etc/X11/xorg.conf.d/

echo "Done with monitor setup (you may want to enter Nvidia or AMD GPU config to make further changes)..."
