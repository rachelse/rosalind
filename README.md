# Rosalind
> Solution of [Rosalind problems](http://rosalind.info/problems/) in Rust<br>
> Find me at the [website](https://rosalind.info/users/eunbelivable/)

## How to run
1. specify the corresponding module in `src/main.rs`
   
   e.g. to run the `DNA` module, you should specify the following line in `src/main.rs`
   ```rust
   stronghold::DNA::run(content);
   ```

   - category of problems -> module
     - python village -> `INI`
     - bioinformatics stronghold -> `stronghold`
     - bioinformatics armory -> `armory`
     - bioinformatics textbook track -> `BA*`
   
2. run the following command
    ```bash
    cargo run <path/to/input/data>
    ```
    You can also use the `run.sh` script to run the program
    ```bash
    ./run.sh test # run the program with test data (the test input should be :`data/test.txt`)
    ./run.sh ba1a # run the program with the input data of the problem `BA1A` (the input data should be :`data/ba1a.txt or in ~/Downloads/ba1a.txt`)
    ```

copyright by [Rachel Seongeun Kim](https://github.com/rachelse)