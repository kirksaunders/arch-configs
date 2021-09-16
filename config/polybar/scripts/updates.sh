#!/bin/bash

UPDATES_MAIN=$(checkupdates | wc -l)
UPDATES_AUR=$(checkupdates-aur | wc -l)
UPDATES=$((UPDATES_MAIN + UPDATES_AUR))

if [[ "${UPDATES}" = "0" ]]
then
    echo ""
else
    echo "󰖷 $UPDATES"
fi
