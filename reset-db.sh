#!/bin/bash

warehouse=10

BGREEN='\033[1;32m'
echo -e "${BGREEN}----------------> ${BGREEN}Gen Data"
cd create-data
bash create-data.sh $warehouse

echo -e "${BGREEN}----------------> ${BGREEN}Gen DB"
cd ..
bash gen-db.sh
