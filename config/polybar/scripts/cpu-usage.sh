#!/bin/bash

log="/tmp/cpu-usage-log"

read line </proc/stat
cpu=($line)
unset cpu[0]
idle=${cpu[4]}

total=0
for v in ${cpu[@]}
do
    total=$((total+v))
done

if [[ -f $log ]]
then
    read line <$log
    pcpu=($line)
    pidle=${pcpu[0]}
    ptotal=${pcpu[1]}

    didle=$((idle-pidle))
    dtotal=$((total-ptotal))

    usage=$(echo "scale=1; x=1000*($dtotal-$didle)/$dtotal/10; if(x>0 && x<1) print 0; x" | bc)

    echo "󰄪 $usage%"
else
    echo "󰄪 0%"
fi

echo $idle $total >$log
