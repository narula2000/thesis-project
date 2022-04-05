#!/bin/bash

gunicorn server.wsgi -w 10 --timeout 5000
