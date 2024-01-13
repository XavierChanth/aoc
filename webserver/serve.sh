#!/usr/bin/env bash

# Fork: https://gist.github.com/XavierChanth/3b027e0cdab863b6ea861894b4b1bfc4

set -Eeuo pipefail
IFS=$'\n\t'

if [ "$#" -ne 1 ]; then
  echo "usage: ./serve <year>" >&2
  exit 1
fi

year=$1

echo "http://localhost:8080/$year"

python3 -m http.server 8080