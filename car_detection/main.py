import os.path

from car_detection.crop_image import crop_all_spots, get_polygons_from_file
from car_detection.detect_car import check_spots
from parking_manager.compare_spots import update_slot_status

if __name__ == "__main__":
    spots_coords_file_path = './parking_spots_coordinates.txt'
    camera_footage = './images/image.jpg'

    if not os.path.isfile(spots_coords_file_path):
        print('Could not find the coordinates file. Exiting...')

    spots_coords = get_polygons_from_file(
        spots_coords_file_path)

    crop_all_spots(camera_footage, spots_coords)

    occupied_spots = check_spots()

    update_slot_status(occupied_spots)
