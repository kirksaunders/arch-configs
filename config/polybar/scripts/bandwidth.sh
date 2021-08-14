#!/bin/bash

log="/tmp/bandwidth-log"

interface=$1

read rx < "/sys/class/net/${interface}/statistics/rx_bytes"
read tx < "/sys/class/net/${interface}/statistics/tx_bytes"
now=$(date +%s%3N)

if [[ -f $log ]]
then
    read line < $log
    words=($line)

    prx=${words[0]}
    ptx=${words[1]}
    ptime=${words[2]}
    
    dt=$((now-ptime))

    [[ $dt -gt 0 ]] || exit

    down=$(((rx-prx) / dt * 1000))
    up=$(((tx-ptx) / dt * 1000))

    echo -n "%{T4}󰀂%{T-} "
    down_kib=$(( $down >> 10 ))
    if [[ "$down" -gt 1048576 ]]
    then
        printf '%sM' "`echo "scale=1; $down_kib / 1024" | bc`"
    else
        echo -n "${down_kib}K"
    fi

    echo -n " : "
    up_kib=$(( $up >> 10 ))
    if [[ "$up" -gt 1048576 ]]
    then
        printf '%sM' "`echo "scale=1; $up_kib / 1024" | bc`"
    else
        echo -n "${up_kib}K"
    fi
    echo ""
else
    echo "%{T4}󰀂%{T-} 0K : 0K"
fi

echo "$rx $tx $now" > $log
