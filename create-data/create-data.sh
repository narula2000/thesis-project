#!/bin/bash

make

mkdir database-data
rm -rf database-data/*

./tpcc-generator $1 database-data

cp -r database-data /home/vm/
