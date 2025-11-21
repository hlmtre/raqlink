# an example using requests to the app

import requests


# post to the url shortener
url = "http://localhost:8192/new"
payload = "http://some.long.domain.tld/directory/structure"

r = requests.post(url, data=payload)
print(r.text)

# and to the image hosting service - this doesn't work for some reason :(

url = "http://localhost:8192/new_image"
headers = {
    'Additional': 'Header',
}
files = {
    'additional': 'image',
    'image': open('random-pictures-MR0G79.jpg', 'rb'),
}
#payload = open(sys.argv[1], "rb")

#print(files)

r = requests.post(url, headers=headers, files=files)
print(r.text)
