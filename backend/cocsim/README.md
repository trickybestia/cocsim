## Benchmarks

To run benchmarks:

```bash
# cd to project root
cd backend/cocsim
TEST_MAPS_PATH="../../test_maps" cargo bench
```

## Fuzzing

At first:

```bash
cargo install cargo-fuzz
cargo install rustfilt
rustup component add llvm-tools-preview
```

To run fuzzing in debug mode:

```bash
# cd to project root
cd backend/cocsim
cargo fuzz run -s none -j $(nproc) fuzz
```

To generate coverage report for [fuzz.rs](./fuzz/fuzz_targets/fuzz.rs) [fuzz_coverage.html](./fuzz/fuzz_coverage.html):

```bash
# cd to project root
cd backend/cocsim
cargo fuzz coverage --target-dir ../target -s none fuzz
llvm-cov show --instr-profile=fuzz/coverage/fuzz/coverage.profdata ../target/*/release/fuzz --name-regex "cocsim" --Xdemangler=rustfilt --use-color | aha > fuzz/fuzz_coverage.html
```
