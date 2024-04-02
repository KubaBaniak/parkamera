import cv2
import numpy as np


def get_polygons_from_file(filename: str) -> list:
    polygons = []
    with open(filename, 'r') as file:
        for line in file:
            pairs = line.strip().split(',')
            polygon = [list(map(int, pair.split()))
                       for pair in pairs if pair.strip()]
            polygons.append(polygon)
    return polygons


def crop_image(slot_id: int, polygon: list, camera_img: list) -> None:
    pts = np.array(polygon)
    rect = cv2.boundingRect(pts)
    x, y, w, h = rect
    cropped = camera_img[y:y+h, x:x+w].copy()
    pts = pts - pts.min(axis=0)
    mask = np.zeros(cropped.shape[:2], np.uint8)
    cv2.drawContours(mask, [pts], -1, (255, 255, 255), -1, cv2.LINE_AA)
    dst = cv2.bitwise_and(cropped, cropped, mask=mask)
    cv2.imwrite(f'images/slots/slot_{slot_id}.png', dst)


def crop_all_slots(base_image_filename: str, polygons: list):
    img = cv2.imread(base_image_filename)
    for (i, polygon) in enumerate(polygons):
        crop_image(i, polygon, img)
