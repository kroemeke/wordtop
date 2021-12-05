#!/bin/bash -x

mkdir books

for book in $(seq 1 100)
do
 curl -s -o books/${book}.txt https://www.gutenberg.org/files/${book}/${book}-0.txt
 sleep 0.5
done
