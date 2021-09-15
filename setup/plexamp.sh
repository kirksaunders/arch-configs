#!/bin/bash

set -e

echo "Starting Plexamp setup..."

if [ "$1" != "update" ]
then
    echo "Installing Flatpak..."
    sudo pacman -S flatpak
    flatpak remote-add --if-not-exists flathub https://dl.flathub.org/repo/flathub.flatpakrepo

    echo "Installing Plexamp..."
    flatpak install --or-update flathub plexamp

    echo "Installing dependencies for Plexamp launcher wrapper..."
    sudo pacman -S xorg-xwininfo jq
fi

echo "Installing Plexamp launcher wrapper..."
sudo cp extra/plexamp.sh /usr/local/bin/
if [ "$1" != "update" ]
then
    sudo cp /var/lib/flatpak/exports/share/applications/com.plexamp.Plexamp.desktop /var/lib/flatpak/exports/share/applications/com.plexamp.Plexamp.desktop.backup
    sudo sed -i 's/Exec=[^\n]\+/Exec=\/usr\/local\/bin\/plexamp.sh/' /var/lib/flatpak/exports/share/applications/com.plexamp.Plexamp.desktop
fi

echo "Done with Plexamp setup!"
