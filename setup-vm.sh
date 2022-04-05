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
sudo systemctl enable docker.service --now
sudo systemctl enable ssh --now

echo -e "${BGREEN}----------------> ${BGREEN}Install Docker Compose"
# Get latest docker compose released tag
COMPOSE_VERSION=$(curl -s https://api.github.com/repos/docker/compose/releases/latest | grep 'tag_name' | cut -d\" -f4)

# Install docker-compose
sudo sh -c "curl -L https://github.com/docker/compose/releases/download/${COMPOSE_VERSION}/docker-compose-`uname -s`-`uname -m` > /usr/local/bin/docker-compose"
sudo chmod +x /usr/local/bin/docker-compose

echo -e "${BGREEN}----------------> ${BGREEN}Mange Docker Permission"
sudo chown $USER /var/run/docker.sock

sudo groupadd docker
sudo usermod -aG docker $USER

echo -e "${BGREEN}----------------> ${BGREEN}Install API Reqs"
cd server
bash get-req.sh
