#!/bin/bash

rm -rf ../images/image_history/*
rm -rf ../images/spots/*
rm -rf ../images/base_img/*
rm ../parking_spots_coordinates.txt
touch ../parking_spots_coordinates.txt
cd ../../backend && sea-orm-cli migrate fresh

cd -

rpicam-still --output ../images/base_img.jpg
python ../setup.py
