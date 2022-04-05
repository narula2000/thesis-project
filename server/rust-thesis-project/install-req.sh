#!/bin/bash

sudo apt update && sudo apt upgrade -y
sudo apt install build-essential -y
curl https://sh.rustup.rs -sSf | sh -s -- --profile default
