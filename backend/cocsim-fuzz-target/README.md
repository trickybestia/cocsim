To run fuzzing in debug mode:

```bash
# cd to project root
cd backend/cocsim-fuzz-target
cargo afl build
cargo afl fuzz -i in -o out -t 1000 ../target/debug/cocsim-fuzz-target
```
