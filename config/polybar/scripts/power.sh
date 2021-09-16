#!/bin/bash

mem="/tmp/power-mem"

notifier="ipc -s=/tmp/power-in client -q"

button_pre="%{A1:echo 'toggle-open' | $notifier:}"
button_font="%{T5}"
button="󰐥"
logout_pre="%{A1:echo 'logout' | $notifier:}"
logout="Logout"
reboot_pre="%{A1:echo 'reboot' | $notifier:}"
reboot="Reboot"
shutdown_pre="%{A1:echo 'shutdown' | $notifier:}"
shutdown="Shutdown"
suffix="%{A}"
separator=" | "

closed_text="$button_pre$button_font$button%{T-}$suffix"
opened_text="$button_pre$button_font$button%{T-}$suffix$separator$logout_pre$logout$suffix$separator$reboot_pre$reboot$suffix$separator$shutdown_pre$shutdown$suffix"
animation_json="{\"Concatenation\":[{\"Tag\":{\"prefix\":\"$button_pre\",\"suffix\":\"$suffix\",\"content\":{\"Tag\":{\"prefix\":\"$button_font\",\"suffix\":\"%{T-}\",\"content\":{\"Raw\":\"$button\"}}}}},{\"Raw\":\"$separator\"},{\"Tag\":{\"prefix\":\"$logout_pre\",\"suffix\":\"$suffix\",\"content\":{\"Raw\":\"$logout\"}}},{\"Raw\":\"$separator\"},{\"Tag\":{\"prefix\":\"$reboot_pre\",\"suffix\":\"$suffix\",\"content\":{\"Raw\":\"$reboot\"}}},{\"Raw\":\"$separator\"},{\"Tag\":{\"prefix\":\"$shutdown_pre\",\"suffix\":\"$suffix\",\"content\":{\"Raw\":\"$shutdown\"}}}]}"

toggle_open=0
force_open=0
force_close=0

if read input
then
    case $input in
        "open")
            force_open=1
            ;;
        "close")
            force_close=1
            ;;
        "toggle-open")
            toggle_open=1
            ;;
        "logout")
            rm -f $mem
            i3-msg exit
            exit
            ;;
        "reboot")
            rm -f $mem
            systemctl reboot
            exit
            ;;
        "shutdown")
            rm -f $mem
            systemctl poweroff
            exit
            ;;
    esac
fi

open=0

if [[ -f $mem ]]
then
    read last <$mem

    open=$last
fi

if [ $open != 0 ] && [ $open != 1 ]
then
    open=0
fi

if [ $toggle_open = 1 ]
then
    [[ $open = 0 ]] && open=1 || open=0
elif [ $force_open = 1 ]
then
    open=1
elif [ $force_close = 1 ]
then
    open=0
fi

if [ ! -z $last ] && [ $open -ne $last ]
then
    if [ $open = 1 ]
    then
        i3-msg mode "power" &>/dev/null
        printf "%s\n" "$animation_json" | text-animator -c=1 -d=0.005 -s=2 forward
    else
        i3-msg mode "default" &>/dev/null
        printf "%s\n" "$animation_json" | text-animator -c=1 -d=0.005 -s=2 reverse
    fi
else
    if [ $open = 1 ]
    then
        printf "%s\n" "$opened_text"
    else
        printf "%s\n" "$closed_text"
    fi
fi

echo $open >$mem
