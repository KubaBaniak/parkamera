import math
import os
import re

import cv2
from ultralytics import YOLO

MODEL = YOLO('yolov8n.pt')
NAMES = MODEL.names


def is_car_on_image(image_filename: str) -> bool:
    """
    Check if car is in the given image
    
    Args:
    image_filename (str): filename to the cropped image of given slot
    
    Returns:
    Boolean: True - car detected | False - car not detected
    """

    img = cv2.imread(image_filename)
    results = MODEL(img, classes=2)
    for r in results:
        for c in r.boxes.cls:
            return 'car' == NAMES[int(c)]


def check_spots() -> list[int]:
    """
    Checks all spots whether there are cars.
    
    Returns:
    List (int): list of slot id's where car was detected.
    """

    dir_path = './images/spots'
    taken_spots = []
    files = os.listdir(dir_path)
    for file in files:
        file_path = os.path.join(dir_path, file)
        if is_car_on_image(file_path):
            match = re.search(r'\d+', file)
            number = match.group() if match else Exception('Wrong spots name')
            taken_spots.append(int(number))
    return taken_spots
