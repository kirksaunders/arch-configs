#!/bin/bash

sep='`'

query_info() {
    playerctl metadata --format "{{ uc(status) }}$sep{{ artist }}$sep{{ title }}$sep" --follow 2>/dev/null
}

coproc fd { query_info; }

while read -u "${fd[0]}" line
do
    if [ -z "$line" ]
    then
        echo ""
    else
        readarray -td "$sep" data <<<"$line"
        status=${data[0]}
        artist=${data[1]}
        title=${data[2]}

        echo -n "%{T6}%{A1:playerctl previous:}󰙤%{A}%{A1:playerctl play-pause:}"
        if [ "$status" = "PAUSED" ] || [ "$status" = "STOPPED" ]
        then
            echo -n "󰐍"
        else
            echo -n "󰏦"
        fi
        echo -n "%{A}%{A1:playerctl next:}󰙢%{A}%{T-} "

        if [ ! -z "$artist" ]
        then
            printf "%s - " "$artist"
        fi

        printf "%s\n" "$title"
    fi
done