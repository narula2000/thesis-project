#!/bin/bash

gunicorn server.wsgi -w 10 --timeout 5000 -b 0.0.0.0:8000
