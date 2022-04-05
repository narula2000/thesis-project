#!/bin/bash

BGREEN='\033[1;32m'

sudo chown -R $USER:$USER .

echo -e "${BGREEN}----------------> ${BGREEN}Clear Docker Compose"
docker-compose down

echo -e "${BGREEN}----------------> ${BGREEN}Clear Database"
sudo rm -rf /tmp/postgres-db

echo -e "${BGREEN}----------------> ${BGREEN}Init DB"
docker-compose -p javascript --file database-intializer.yml up --build
