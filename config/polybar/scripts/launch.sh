#!/bin/bash

# Terminate already running bar instances
killall -q polybar

# Wait until the processes have been shut down
while pgrep -u $UID -x polybar >/dev/null; do sleep 0.1; done

rdp=$(xrandr | grep rdp0)
nomachine=$(xrandr | grep nxoutput0)

# Launch Polybar, using default config location ~/.config/polybar/config
if [[ ! -z "$rdp" ]];
then
    polybar xrdp &
elif [[ ! -z "$nomachine" ]];
then
    polybar nomachine &
else
    polybar left &
    polybar right &
fi
