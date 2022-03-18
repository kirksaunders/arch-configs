#!/bin/bash

CORE="${1}"
OUTPUT=$(sensors -Au 2>/dev/null)

IFS=$'\n'
DATA=$(echo "${OUTPUT}" | egrep -A2 "${CORE}")
CURRENT=$(echo "${DATA}" | grep -Po "(?<=_input: )([0-9]+)")
#HIGH=$(echo "${DATA}" | grep -Po "(?<=_max: )([0-9]+)")
#CRIT=$(echo "${DATA}" | grep -Po "(?<=_crit: )([0-9]+)")

echo "%{T4}󰔄%{T-} ${CURRENT}"
