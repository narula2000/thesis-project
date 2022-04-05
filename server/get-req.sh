#!/bin/bash

for D in *; do
    if [ -d "${D}" ]; then
        echo "Updating... ${D}"
        cd $D
        bash  install-req.sh
        cd ..
        echo "Done... ${D}"
    fi
done
