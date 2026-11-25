#!/usr/bin/env bash

set -Eeuo pipefail
# set -x

sudo ls >/dev/null

hyperfine -S --warmup 2 --min-runs 10 --metrics time \
  --export-json results.json --export-markdown results.md \
  --prepare 'sync; echo 3 | sudo tee /proc/sys/vm/drop_caches' \
  -n 'marc21-learn' './marc21-learn-validate.py ~/tmp/isbn13.csv' \
  -n 'isbnlib' './isbnlib-validate.py ~/tmp/isbn13.csv' \
  -n 'isbn-pure' './isbn-pure-validate.py ~/tmp/isbn13.csv' \
  -n 'is-isbn' './is-isbn-validate.py ~/tmp/isbn13.csv' \
  -n 'pydantic' './pydantic-isbn-validate.py ~/tmp/isbn13.csv' \
  -n 'pyisbn' './pyisbn-validate.py ~/tmp/isbn13.csv'
