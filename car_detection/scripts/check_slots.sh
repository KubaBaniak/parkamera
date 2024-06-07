#!/bin/bash

time_stamp=$(date +%F_%H-%M-%S)

rpicam-still --output "../images/image_history/parking_$time_stamp.jpg"

. ../venv/bin/activate && python3 ../main.py

