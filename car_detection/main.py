import os.path

from car_detection.crop_image import crop_all_slots, get_polygons_from_file
from car_detection.detect_car import check_slots

if __name__ == "__main__":
    slots_coords_file_path = './parking_slots_coordinates.txt'
    camera_footage = './images/image.jpg'

    if not os.path.isfile(slots_coords_file_path):
        print('whaat')

    slots_coords = get_polygons_from_file(
        slots_coords_file_path)

    crop_all_slots(camera_footage, slots_coords)

    print(check_slots())
