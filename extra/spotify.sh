#!/bin/bash

pid=$(ps -ef | grep "spotify" | awk '{ if ($8 == "/opt/spotify/spotify") { print $2; exit } }')

if [ -z "$pid" ]
then
    i3-msg "exec --no-startup-id spotify"
else
    ids=($(xwininfo -tree -root -int | grep spotify | awk '{print $1}'))

    for id in "${ids[@]}"
    do
        p=$(xprop -id $id _NET_WM_PID | awk '{print $3}')
        name=$(xprop -id $id WM_NAME | awk '{print $3}')
        if [ "$p" = "$pid" ] && [ "$name" != "\"spotify\"" ]
        then
            workspace=$(i3-msg -t get_workspaces | jq -r '.[] | select(.focused==true).name')
            xdotool windowmap $id
            i3-msg "[id=$id] move to workspace number $workspace"
            break
        fi
    done
fi