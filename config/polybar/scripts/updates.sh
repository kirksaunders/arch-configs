#!/bin/bash

UPDATES=$(checkupdates | wc -l)

if [[ "${UPDATES}" = "0" ]]
then
    echo ""
else
    echo "󰖷 $UPDATES"
fi
