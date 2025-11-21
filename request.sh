#!/usr/bin/env bash

# test the URL shortener
curl -v -d "https://some.long.tld/subdirectory/here" -X POST http://localhost:8193/new
# testing image rehosting
curl --header 'Additional: Header' --form additional=image --form image=@random-pictures-MR0G79.jpg http://localhost:8193/new_image
#curl --header 'Additional: Header' --form additional=image --form image=@random-pictures-MR0G79.jpg https://u.aql.ink/new_image
# get one of your uploaded images
#http://localhost:8193/i/<unique_name>
