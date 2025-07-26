To run in debug mode:

```bash
# cd to project root
RUST_LOG=tower_http=trace,webserver=debug RUST_BACKTRACE=1 cargo run --manifest-path=backend/webserver/Cargo.toml
```
