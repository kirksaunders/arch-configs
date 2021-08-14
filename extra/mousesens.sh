#! /bin/sh

sleep .1

mice=$(xinput --list --short | grep -i Logitech)
reg="id=([0-9]+)"
sens="0.6"

IFS=$'\n'
for m in $mice
do
    if [[ $m =~ $reg ]]
    then
        id="${BASH_REMATCH[1]}"
        xinput set-prop $id "Coordinate Transformation Matrix" $sens 0 0 0 $sens 0 0 0 1
    fi
done
