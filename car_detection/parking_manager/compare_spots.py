import requests
import json

def update_slot_status(occupied_spots):
    try:
        response = requests.get('http://localhost:3000/cars?current=true')
        response.raise_for_status()
        json_string = response.content.decode('utf-8')
        current_cars = json.loads(json_string)
        
        if current_cars:
            remove_departed_cars(current_cars, occupied_spots)
        else:
            print("No cars to process for departure.")
            
        if occupied_spots:
            update_arrived_cars(current_cars, occupied_spots)
        else:
            print("No new cars to notify as arrived.")
            
    except requests.exceptions.RequestException as e:
        print(f"Failed to fetch current cars data. Error: {e}")
    except json.JSONDecodeError:
        print("Failed to decode the response from the server. It might be empty or malformed.")

def remove_departed_cars(current_cars, occupied_spots):
    """
    Updates the list of occupied spots by removing cars that have departed.
    
    Args:
    - current_cars (list of dict): Current cars in the parking lot.
    - occupied_spots (list of int): List of currently occupied spot IDs.
    
    Returns:
    - list of int: Updated list of occupied spot IDs.
    """

    for car in current_cars:
        if car['spot_id'] not in occupied_spots:
            body = { 'spot_id': car['spot_id'] }
            req = requests.patch('http://localhost:3000/left', json=body)
            print(f"Car left from spot {car['spot_id']}. Status code: {req.status_code}")


def update_arrived_cars(current_cars, occupied_spots):
    """
    Notifies the system about newly arrived cars.
    
    Args:
    - occupied_spots (list of dict): List of spots newly taken by cars.
    """
    for spot_id in occupied_spots:
        if spot_id not in [car['spot_id'] for car in current_cars]:
            body = {'spot_id': spot_id}
            req = requests.post('http://localhost:3000/arrived', json=body)
            print(f"Car arrived at spot {spot_id}. Status code: {req.status_code}")

