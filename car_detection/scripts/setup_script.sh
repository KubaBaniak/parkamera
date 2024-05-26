#!/bin/bash

cd rm -rf ../images/image_history/*
cd rm -rf ../images/spots/*
cd rm -rf ../images/base_img/*
cd ../../backend sea-orm-cli migrate fresh


rpicam-still --output ../images/base_img.jpg
python ../setup.py
