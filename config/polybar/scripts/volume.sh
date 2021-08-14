#!/bin/bash

if read input
then
    case $input in
        "pavucontrol")
            i3-msg exec pavucontrol &>/dev/null
            ;;
        "up")
            pamixer -i 5
            ;;
        "down")
            pamixer -d 5
            ;;
        "toggle")
            pamixer -t
            ;;
    esac
fi

volume=$(pamixer --get-volume)

if [ $(pamixer --get-mute) = "true" ]
then
    printf "󰝟"
elif [ $volume -ge 75 ]
then
    printf "󰕾"
elif [ $volume -ge 25 ]
then
    printf "󰖀"
else
    printf "󰕿"
fi

printf " "
if [ $volume -ge 100 ]
then
    printf "$volume%%"
elif [ $volume -ge 10 ]
then
    printf " $volume%%"
else
    printf "  $volume%%"
fi
echo ""

