import json
import requests

occupied_spots = [0]

def remove_departed_cars(current_cars, occupied_spots):
    """
    Updates the list of occupied spots by removing cars that have departed.
    
    Args:
    - current_cars (list of dict): Current cars in the parking lot.
    - occupied_spots (list of int): List of currently occupied spot IDs.
    
    Returns:
    - list of int: Updated list of occupied spot IDs.
    """

    departed_cars = []
    
    for car in current_cars:
        if car['spot_id'] not in occupied_spots:
            body = { 'spot_id': car['spot_id'] }
            req = requests.patch('http://localhost:3000/left', json=body)
            print(f"Car left from spot {car['spot_id']}. Status code: {req.status_code}")

def update_arrived_cars(occupied_spots):
    """
    Notifies the system about newly arrived cars.
    
    Args:
    - occupied_spots (list of dict): List of spots newly taken by cars.
    """
    for spot in occupied_spots:
        body = {'spot_id': spot}
        req = requests.post('http://localhost:3000/arrived', json=body)
        print(f"Car arrived at spot {spot}. Status code: {req.status_code}")

response = requests.get('http://localhost:3000/cars?current=true')
json_string = response.content.decode('utf-8')
current_cars = json.loads(json_string)

remove_departed_cars(current_cars, occupied_spots)
update_arrived_cars(occupied_spots)

