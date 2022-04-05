#!/bin/bash

sudo apt update && sudo apt upgrade -y
sudo apt install software-properties-common -y
sudo apt install python3-pip -y
pip install gunicorn django psycopg2-binary djangorestframework django-stubs django-extensions
