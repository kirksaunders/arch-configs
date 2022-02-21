#!/bin/bash

sep='`'

query_info() {
    while [ 1 ];
    do
        timeout 1 playerctl metadata --format "{{ uc(status) }}$sep{{ artist }}$sep{{ title }}$sep" --follow 2>/dev/null | tee /tmp/media-tmp
        out="$(< /tmp/media-tmp)"

        if [[ -z "$out" ]];
        then
            playerctl metadata --format "{{ uc(status) }}$sep{{ artist }}$sep{{ title }}$sep" 2>/tmp/media-tmp-error
            err="$(< /tmp/media-tmp-error)"
            if [[ "$err" == "No players found" ]];
            then
                echo ""
            fi
        fi
    done
}

coproc fd { query_info; }

while read -u "${fd[0]}" line
do
    if [[ -n "${line/[ ]*\n/}" ]]
    then
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
    else
        echo ""
    fi
done