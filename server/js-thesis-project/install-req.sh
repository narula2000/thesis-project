#!/bin/bash

sudo apt update && sudo apt upgrade -y
sudo apt install build-essential -y
curl -sL https://deb.nodesource.com/setup_16.x -o /tmp/nodesource_setup.sh
sudo bash /tmp/nodesource_setup.sh
sudo apt install nodejs -y
