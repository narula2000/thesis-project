#!/bin/bash

workers=10

gunicorn server.wsgi -w $workers --timeout 5000 -b 0.0.0.0:8000
