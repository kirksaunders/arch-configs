#!/bin/bash

workspaces=$(i3-msg -t get_workspaces)

focused=$(printf "%s\n" "$workspaces" | jq -r '.[] | select(.focused==true).name')
mapfile -t all_workspaces < <(printf "%s\n" "$workspaces" | jq -r '.[] | .name')

first=1
for workspace in "${all_workspaces[@]}"
do
    if [ "$first" = "1" ]; then
        first=0
    else
        printf "%%{T8} | %%{T-}"
    fi
    if [ "$workspace" == "$focused" ]; then
        printf "%%{T9}$workspace%%{T-}"
    else
        printf "%%{A1:i3-msg workspace number $workspace; echo 'update' | ipc -s=/tmp/i3-workspace-in client -q:}%%{T7}$workspace%%{T-}%%{A}"
    fi
done

echo ""