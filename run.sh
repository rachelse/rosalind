#!/bin/bash

if [ $1 == "test" ]; then 
    RUSTFLAGS=-Awarnings cargo run data/sample.txt
elif [ -f ~/Downloads/rosalind_$1".txt" ]; then
    RUSTFLAGS=-Awarnings cargo run ~/Downloads/rosalind_ba3i.txt
elif [ -f data/rosalind_$1".txt" ]; then
    RUSTFLAGS=-Awarnings cargo run data/rosalind_ba3g.txt
else
    echo "Input File not found"
fi
#RUSTFLAGS=-Awarnings cargo run data/extra.txt
