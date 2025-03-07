#!/bin/bash

if [ $1 == "test" ]; then 
    RUSTFLAGS=-Awarnings cargo run data/test.txt
elif [ -f ~/Downloads/rosalind_$1".txt" ]; then
    RUSTFLAGS=-Awarnings cargo run ~/Downloads/rosalind_$1.txt
elif [ -f data/rosalind_$1".txt" ]; then
    RUSTFLAGS=-Awarnings cargo run data/rosalind_$1.txt
else
    echo "Input File not found"
fi
#RUSTFLAGS=-Awarnings cargo run data/extra.txt
