#!/bin/bash

echo "Starting Spotify setup..."

echo "Installing Spotify..."
paru -S spotify

echo "Installing dependencies for Spotify launcher wrapper..."
sudo pacman -S xorg-xwininfo

echo "Installing Spotify launcher wrapper..."
sudo cp extra/spotify.sh /usr/local/bin/
sudo cp /usr/share/applications/spotify.desktop /usr/share/applications/spotify.desktop.backup
sudo sed -i 's/TryExec=spotify/TryExec=\/usr\/local\/bin\/spotify.sh/' /usr/share/applications/spotify.desktop
sudo sed -i 's/Exec=spotify %U/Exec=\/usr\/local\/bin\/spotify.sh/' /usr/share/applications/spotify.desktop

echo "Done with Spotify setup!"