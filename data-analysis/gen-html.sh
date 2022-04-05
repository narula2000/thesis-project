#!/bin/bash

rm htmls/*

jupyter nbconvert --to html 'Describe Data.ipynb'
jupyter nbconvert --to html 'Plot Difference By CPU.ipynb'
jupyter nbconvert --to html 'Plot Difference By RAM.ipynb'
jupyter nbconvert --to html 'Plot Difference By Warehouse.ipynb'

mv *.html htmls
