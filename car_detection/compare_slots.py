import requests

r = requests.get('http://localhost:3000/cars')

print(r.content)
