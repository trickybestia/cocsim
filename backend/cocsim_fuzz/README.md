At first:

```bash
cargo install cargo-fuzz
cargo install rustfilt
rustup component add llvm-tools-preview
```

To run fuzzing in debug mode:

```bash
# cd to project root
cd backend
cargo fuzz run --fuzz-dir cocsim_fuzz -s none -j $(nproc) fuzz
```

To generate coverage report [coverage.html](../coverage.html):

```bash
# cd to project root
cd backend
cargo fuzz coverage --fuzz-dir cocsim_fuzz -s none fuzz
llvm-cov show --instr-profile=cocsim_fuzz/coverage/fuzz/coverage.profdata target/*/coverage/*/release/fuzz --name-regex "cocsim" --Xdemangler=rustfilt --use-color | aha > coverage.html
```
