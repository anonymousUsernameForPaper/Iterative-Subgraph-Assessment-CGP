# Extending Cartesian Genetic Programming via Iterative Subgraph Assessment
Code and Benchmarks for the paper: Extending Cartesian Genetic Programming via Iterative Subgraph Assessment

# Rust
The code is written in Rust only.  
For installation, see: https://github.com/rust-lang/rust/blob/master/README.md

# Building
You have to build everything yourself. You will need a working `Rust` and `Cargo` setup. [Rustup](https://rustup.rs/) is the simplest way to set this up on either Windows, Mac or Linux.

Once the prerequisites have been installed, compilation on your native platform is as simple as running the following in a terminal:

```
cargo build --release
```


# Usage
Run the build executable on your machine via:
```
./target/release/cgp
```
or 
```
./target/release/cgp.exe
```

Benchmarks must be downloaded from: [https://space.mit.edu/home/tegmark/aifeynman.html](https://space.mit.edu/home/tegmark/aifeynman.html)  
All files must be placed into the same folder as the executable into a folder called `feynman_files`.  

Outputs will be placed into a folder called
`Experiments_Output`

You can configure the run via following command line arguments:
- `run-id`
  - The ID of the run
  - Only important for saving results
  - default: 0
- `nbr-nodes`
  - the number of computational nodes for CGP
  - default: 500
- `mutation-type`  
  - `0` or `1`   
  - 0: Single Active Mutation
  - 1: Probabilistic Mutation (Point Mutation)
  - default: 0
- `mutation-rate`  
  - Mutation Rate for Probabilistic Mutation (Point Mutation)
  - default: 0.2
- `bend-connection-nodes`  
  - `0` or `1`   
  - 0: Standard CGP without extension
  - 1: Includes ISA-CGP 
  - default: 0
- `dataset-name`  
  - file-name of Feynman Benchmark file
  - default: 0
- `budget-type`  
  - `0`, `1`, or `2`   
  - 0: time based budget (12 hours)
  - 1: Fitness evaluations (1,000,000 fitness evaluations)
  - 2: Training iterations (250,000 iterations)
  - default: 0
