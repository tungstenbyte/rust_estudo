#!/bin/bash

# Procurar todos os arquivos .rs e .toml a partir da pasta atual
find . -type f \( -name "*.rs" -o -name "*.toml" \) | while read -r file; do
    echo "---------------------------------------------"
    echo "$file"
    cat "$file"
done
