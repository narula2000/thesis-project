#!/bin/bash

warehouse=10

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

echo -e "${BGREEN}----------------> ${BGREEN}Install Postgres"
sudo apt update && sudo apt upgrade -y
sudo apt install build-essential -y postgresql-client-common postgresql libpq-dev -y
sudo systemctl enable ssh --now

echo -e "${BGREEN}----------------> ${BGREEN}Config Postgres"
sudo rm /etc/postgresql/12/main/pg_hba.conf
sudo cp postgres-config.conf /etc/postgresql/12/main/pg_hba.conf
sudo bash -c "echo \"listen_addresses = '*'\" >> /etc/postgresql/12/main/postgresql.conf"
sudo bash -c "echo \"log_statement = 'ddl'\" >> /etc/postgresql/12/main/postgresql.conf"
sudo bash -c "echo \"max_wal_size = 5GB\" >> /etc/postgresql/12/main/postgresql.conf"

echo -e "${BGREEN}----------------> ${BGREEN}Start Postgres"
sudo pg_ctlcluster 12 main start
sudo service postgresql restart

echo -e "${BGREEN}----------------> ${BGREEN}Gen Data"
cd create-data
bash create-data.sh $warehouse

echo -e "${BGREEN}----------------> ${BGREEN}Gen DB"
cd ..
bash gen-db.sh
