At first:

```bash
cargo install cargo-fuzz
```

To run fuzzing in debug mode:

```bash
# cd to project root
cd backend/cocsim
cargo fuzz run --fuzz-dir ../cocsim_fuzz -s none fuzz_validate
```
