# an example using requests to the app

import requests

url = "http://localhost:8192/new"
payload = "http://some.long.domain.tld/directory/structure"

r = requests.post(url, data=payload)
print(r.text)
