import requests
import json

# Define the album data
albums = [
    {
        "album_id": 0,
        "name": "Rust In Peace",
        "tracks": 12,
        "artists": "Megadeth",
        "genres": "Metal",
        "year": 1992,
        "rating": 10,
        "comment": "Thrash masterpiece!"
    },
    {
        "album_id": 1,
        "name": "Master of Puppets",
        "tracks": 8,
        "artists": "Metallica",
        "genres": "Thrash Metal",
        "year": 1986,
        "rating": 9,
        "comment": "Iconic metal album."
    },
    {
        "album_id": 2,
        "name": "Back in Black",
        "tracks": 10,
        "artists": "AC/DC",
        "genres": "Hard Rock",
        "year": 1980,
        "rating": 4,
        "comment": "One of the best-selling albums of all time."
    },
    {
        "album_id": 3,
        "name": "Nevermind",
        "tracks": 13,
        "artists": "Nirvana",
        "genres": "Grunge",
        "year": 1991,
        "rating": 10,
        "comment": "A defining album of the 90s."
    },
    {
        "album_id": 4,
        "name": "The Wall",
        "tracks": 26,
        "artists": "Pink Floyd",
        "genres": "Progressive Rock",
        "year": 1979,
        "rating": 10,
        "comment": "A rock opera and concept album."
    },
    {
        "album_id": 5,
        "name": "Appetite for Destruction",
        "tracks": 12,
        "artists": "Guns N Roses",
        "genres": "Hard Rock",
        "year": 1987,
        "rating": 2,
        "comment": "Debut album with massive success."
    }
]

# Define the URL for the localhost server
url = "http://localhost:8080/albums"

# Send POST requests for each album
for album in albums:
    response = requests.post(url, json=album)
    if response.status_code == 200:
        print(f"Successfully added album: {album['name']}")
    else:
        print(f"Failed to add album: {album['name']}. Status code: {response.status_code}, Response: {response.text}")
