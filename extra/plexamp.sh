#!/bin/bash

id=$(xwininfo -tree -root -int | grep Plexamp | awk '{ if ($2 == "\"Plexamp\":") { print $1; exit } }')

if [ -z "$id" ]
then
    i3-msg "exec --no-startup-id /usr/bin/flatpak run --branch=stable --arch=x86_64 --command=plexamp-run --file-forwarding com.plexamp.Plexamp"
else
    workspace=$(i3-msg -t get_workspaces | jq -r '.[] | select(.focused==true).name')
    workspace2=$(i3-msg -t get_tree | jq -r '.nodes[].nodes[].nodes[] | select(.floating_nodes[].nodes[].name=="Plexamp") | .name')
    if [[ "$1" = "toggle" ]] && [[ "$workspace" = "$workspace2" ]]
    then
        xdotool windowunmap $id
    else
        xdotool windowmap $id
        #i3-msg "[id=$id] move to workspace number $workspace"
        i3-msg "[id=$id] move position mouse"
        i3-msg "[id=$id] focus"
    fi
fi