Get Started
=======

1. Clone the repository

```shell
git clone https://github.com/Archfx/hFuzz.git
```

2. Initialize submodules

```shell
git submodule init
git submodule update
```

3. Verify that rust works correctly

Run the following from the main directory (/hFuzz)
```shell
cargo run
```
This is a z3 example to solve a cryptarithmetic problem.

4. Verify that LibAFL works correctly

Run the following from inside a fuzzer example in the fuzzers folder in LibAFL(For example, /hFuzz/LibAFL/fuzzers/baby_fuzzer)
```shell
cargo run
```
