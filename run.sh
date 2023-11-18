#!/bin/bash

# check if rustc is installed
if ! command -v rustc &> /dev/null
then
    echo "rustc could not be found"
    exit
fi

# compile $1 and store in tmp/ with proper name
mkdir -p tmp
rustc $1 -o tmp/$(basename $1 .rs)

# run the compiled file
./tmp/$(basename $1 .rs)
