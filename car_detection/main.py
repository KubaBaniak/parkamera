import os.path

from glob import glob

from car_detection.crop_image import crop_all_spots, get_polygons_from_file
from car_detection.detect_car import check_spots
from parking_manager.compare_spots import update_slot_status

if __name__ == "__main__":
    spots_coords_file_path = './parking_spots_coordinates.txt'
    list_of_images = glob.glob('./images/image_history/*')
    latest_image = max(list_of_images, key=os.path.getctime)

    if not os.path.isfile(spots_coords_file_path):
        print('Could not find the coordinates file. Exiting...')

    spots_coords = get_polygons_from_file(
        spots_coords_file_path)

    crop_all_spots(latest_image, spots_coords)

    occupied_spots = check_spots()

    update_slot_status(occupied_spots)
