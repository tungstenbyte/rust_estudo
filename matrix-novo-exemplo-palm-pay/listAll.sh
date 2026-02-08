#!/bin/bash

find . -type f -name "*.*" | while read -r file; do
    echo "--------------------------------------------------------------"
    echo "$file"
    echo
    cat "$file"
    echo
done
