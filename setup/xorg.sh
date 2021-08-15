#!/bin/bash

set -e

echo "Starting xorg setup..."

if [ $1 == "update" ]
then
    echo "Nothing to do"
    exit 0
fi

echo "Installing xorg server..."
sudo pacman -S xorg-server xorg-xinit xdotool

echo "Installing default xinitrc..."
cp config/.xinitrc ~/

echo "Installing default xprofile..."
echo -e '#!/bin/bash\n' > ~/.xprofile

echo "Adding bin to path..."
echo 'export PATH=$PATH:~/bin/;' >> ~/.xprofile

echo "Adding default mouse pos to xinitrc..."
echo "xdotool mousemove 960 540" >> ~/.xinitrc

echo "Done with xorg setup!"
