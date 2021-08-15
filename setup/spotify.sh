#!/bin/bash

set -e

echo "Starting Spotify setup..."

if [ $1 != "update" ]
then
    echo "Installing Spotify..."
    paru --skipreview --removemake --cleanafter -S spotify

    echo "Installing dependencies for Spotify launcher wrapper..."
    sudo pacman -S xorg-xwininfo jq
fi

echo "Installing Spotify launcher wrapper..."
sudo cp extra/spotify.sh /usr/local/bin/
if [ $1 ~= "update" ]
then
    sudo cp /usr/share/applications/spotify.desktop /usr/share/applications/spotify.desktop.backup
    sudo sed -i 's/TryExec=spotify/TryExec=\/usr\/local\/bin\/spotify.sh/' /usr/share/applications/spotify.desktop
    sudo sed -i 's/Exec=spotify %U/Exec=\/usr\/local\/bin\/spotify.sh/' /usr/share/applications/spotify.desktop
fi

echo "Done with Spotify setup!"
