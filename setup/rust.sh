#!/bin/bash

set -e

echo "Starting rust setup..."

if [ $1 == "update" ]
then
    echo "Nothing to do"
    exit 0
fi

echo "Installing rust..."
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

echo "Done with rust setup!"
