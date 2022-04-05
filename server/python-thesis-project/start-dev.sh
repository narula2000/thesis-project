#!/bin/bash

BGREEN='\033[1;32m'

sudo chown -R $USER:$USER .

echo -e "${BGREEN}----------------> ${BGREEN}Clear Docker Compose"
docker-compose down

echo -e "${BGREEN}----------------> ${BGREEN}Build & Run Docker Compose"
docker-compose -p python up --build
