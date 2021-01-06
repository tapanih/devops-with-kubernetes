#!/bin/bash

if [ $TODO_POST_URL ]
then
  url=$(curl --head https://en.wikipedia.org/wiki/Special:Random | \
    grep 'location: ' | \
    sed -e 's/location: //' -e 's/\r//')
  curl $TODO_POST_URL -H "Content-Type: application/json" \
    --data "{\"content\": \"Read ${url}\"}"
else
  echo "TODO_POST_URL not found"
fi
