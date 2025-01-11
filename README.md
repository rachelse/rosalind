# Rosalind
Solution of [Rosalind problems](http://rosalind.info/problems/) in Rust
Find me in rosalind [here](https://rosalind.info/users/eunbelivable/)

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

copyright by [Rachel Seongeun Kim](https://github.com/rachelse)