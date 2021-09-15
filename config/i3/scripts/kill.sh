#!/bin/bash

winID=$(xdotool getactivewindow)
winClass=$(xprop -id $winID WM_CLASS)

if [[ $winClass = *"Steam"* ]] || [[ $winClass = *"Spotify"* ]] || [[ $winClass = *"Plexamp"* ]]
then
    xdotool windowunmap $winID
else
    i3-msg kill
fi