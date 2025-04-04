#!/bin/bash

input=${1:-"test"}
echo Run Rosalind $input

if [ $input == "test" ]; then 
    RUSTFLAGS=-Awarnings cargo run data/test.txt
elif [ -f ~/Downloads/rosalind_$input".txt" ]; then
    RUSTFLAGS=-Awarnings cargo run ~/Downloads/rosalind_$input.txt
elif [ -f data/rosalind_$input".txt" ]; then
    RUSTFLAGS=-Awarnings cargo run data/rosalind_$input.txt
else
    echo "Input File not found"
fi
#RUSTFLAGS=-Awarnings cargo run data/extra.txt
