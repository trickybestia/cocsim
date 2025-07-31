To run in debug mode:

```bash
# cd to project root
TEST_MAPS_PATH="test_maps" RUST_LOG=tower_http=trace,webserver=debug RUST_BACKTRACE=1 cargo run --manifest-path=backend/webserver/Cargo.toml
```
