#!/bin/bash

if [[ $# -ne 4 ]]; then
    echo 'Too many/few arguments, expecting 4 warehouse cpu ram lang'
    exit 1
fi

mv out.csv out-w$1-c$2-r$3-$4.csv

echo "Done: out-w$1-c$2-r$3-$4.csv"
