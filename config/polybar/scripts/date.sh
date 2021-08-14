#!/bin/bash

DaySuffix() {
    case `date +%-d` in
        1|21|31) echo "st";;
        2|22)    echo "nd";;
        3|23)    echo "rd";;
        *)       echo "th";;
    esac
}

echo "󰸗 $(date +'%a %b %-d')`DaySuffix`"

if read input
then
    if [ "$input" = "calendar" ]
    then
        i3-msg 'exec --no-startup-id gsimplecal' &>/dev/null
    fi
fi
