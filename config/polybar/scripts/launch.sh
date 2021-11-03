#!/bin/bash

# Terminate already running bar instances
killall -q polybar

# Wait until the processes have been shut down
while pgrep -u $UID -x polybar >/dev/null; do sleep 0.1; done

monitor=$(xrandr | grep HDMI-0)

echo $monitor

# Launch Polybar, using default config location ~/.config/polybar/config
if [ -z "$monitor" ];
then
    polybar xrdp &

else
    polybar left &
    polybar right &
fi