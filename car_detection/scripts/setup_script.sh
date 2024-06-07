#!/bin/bash

rm -rf ../images/image_history/*
rm -rf ../images/spots/*
rm -rf ../images/base_img/*
rm ../parking_spots_coordinates.txt
touch parking_spots_coordinates.txt
../../backend && sea-orm-cli migrate fresh


rpicam-still --output ../images/base_img.jpg
python ../setup.py
