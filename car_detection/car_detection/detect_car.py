import math
import os
import re

import cv2
from ultralytics import YOLO

MODEL = YOLO('yolov8n.pt')
NAMES = MODEL.names


def is_car_on_image(image_filename: str) -> bool:
    img = cv2.imread(image_filename)
    results = MODEL(img, classes=2)
    for r in results:
        for c in r.boxes.cls:
            return 'car' == NAMES[int(c)]


def check_slots() -> list[int]:
    dir_path = './images/slots'
    taken_slots = []
    files = os.listdir(dir_path)
    for file in files:
        file_path = os.path.join(dir_path, file)
        if is_car_on_image(file_path):
            match = re.search(r'\d+', file)
            number = match.group() if match else Exception('Wrong slots name')
            taken_slots.append(number)
    return taken_slots
