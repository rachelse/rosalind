#!/bin/bash

input=${1:-"test"}

if [ $input == "test" ]; then 
    RUSTFLAGS=-Awarnings cargo run data/test.txt
elif [ -f ~/Downloads/rosalind_$input".txt" ]; then
    RUSTFLAGS=-Awarnings cargo run ~/Downloads/rosalind_$input.txt
    if [ "$2" == "--remove" ]; then
        rm ~/Downloads/rosalind_$input.txt
    fi
elif [ -f data/rosalind_$input".txt" ]; then
    RUSTFLAGS=-Awarnings cargo run data/rosalind_$input.txt
    if [ "$2" == "--remove" ]; then
        rm data/rosalind_$input.txt
    fi
else
    echo "Input File not found"
fi
#RUSTFLAGS=-Awarnings cargo run data/extra.txt
