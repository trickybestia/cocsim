To run in debug mode:

Simple:

```bash
# cd to project root
TEST_MAPS_PATH="test_maps" RUST_BACKTRACE=1 cargo run --manifest-path=backend/showcase/Cargo.toml --bin=simple
```

Attack optimizer:

```bash
# cd to project root
TEST_MAPS_PATH="test_maps" RUST_BACKTRACE=1 cargo run --manifest-path=backend/showcase/Cargo.toml --bin=attack-optimizer
```
