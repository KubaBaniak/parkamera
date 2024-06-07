import cv2
import numpy as np


def get_polygons_from_file(filename: str) -> list:
    """
    Converts the polygons coordinates from the text file to the 
    list of lists of coordinates.
    Ex. [ [[x1, y1], [x2, y2], [x3, y3], [x4, y4]], ...]
    
    Args:
    filename (string): filename of the coordinates text file in the root 
    directory
    
    Returns:
    list of list of list of 2 ints
    """

    polygons = []
    with open(filename, 'r') as file:
        for line in file:
            pairs = line.strip().split(',')
            polygon = [list(map(int, pair.split()))
                       for pair in pairs if pair.strip()]
            polygons.append(polygon)
    return polygons


def crop_image(slot_id: int, polygon: list, camera_img: list) -> None:
    """
    Crops an image to the size of slot with id: 'slot_id'
    
    Args:
    slot_id (int) 
    polygon (list of list of list of 2 ints)
    camera_img (cv2 image)
    
    Returns:
    Nothing: it saves the cropped image to the images/spots/slot_<slot_id>.png file
    """
    pts = np.array(polygon)
    rect = cv2.boundingRect(pts)
    x, y, w, h = rect
    cropped = camera_img[y:y+h, x:x+w].copy()
    pts = pts - pts.min(axis=0)
    mask = np.zeros(cropped.shape[:2], np.uint8)
    cv2.drawContours(mask, [pts], -1, (255, 255, 255), -1, cv2.LINE_AA)
    dst = cv2.bitwise_and(cropped, cropped, mask=mask)
    cv2.imwrite(f'../images/spots/slot_{slot_id}.png', dst)


def crop_all_spots(base_image_filename: str, polygons: list):
    """
    Crops all the spots on the image
    
    Args:
    base_image_filename (str): filename of the image from camera.
    polygon (list of list of list of 2 ints)
    
    Returns:
    None (look crop_image function)
    """
    img = cv2.imread(base_image_filename)
    for (i, polygon) in enumerate(polygons):
        crop_image(i, polygon, img)
