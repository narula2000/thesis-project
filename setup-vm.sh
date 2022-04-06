#!/bin/bash

BGREEN='\033[1;32m'

echo -e "${BGREEN}----------------> ${BGREEN}Copy Config"
cp tmux.conf ~/.tmux.conf
cp vimrc ~/.vimrc

echo -e "${BGREEN}----------------> ${BGREEN}Edit Bashrc"
echo '' >> ~/.bashrc
echo 'ip="192.168.122."' >> ~/.bashrc
echo 'dbip="192.168.122."' >> ~/.bashrc
echo '' >> ~/.bashrc
echo 'export ip' >> ~/.bashrc
echo 'export dbip' >> ~/.bashrc
echo '' >> ~/.bashrc
echo 'alias check-ip="virsh domifaddr ubuntu"' >> ~/.bashrc
echo 'alias check-ip-db="virsh domifaddr database"' >> ~/.bashrc
echo 'alias vm="ssh vm@${ip}"' >> ~/.bashrc
echo 'alias db="ssh vm@${dbip}"' >> ~/.bashrc
echo 'alias ee="vim ."' >> ~/.bashrc
echo 'alias e="vim"' >> ~/.bashrc
echo 'alias ss="source ~/.bashrc"' >> ~/.bashrc

echo -e "${BGREEN}----------------> ${BGREEN}Install Reqs"
sudo apt update && sudo apt upgrade -y
sudo apt install build-essential libpq-dev -y
sudo systemctl enable ssh --now

echo -e "${BGREEN}----------------> ${BGREEN}Install API Reqs"
cd server
bash get-req.sh
