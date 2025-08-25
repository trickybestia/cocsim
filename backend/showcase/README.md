Simple (debug):

```bash
# cd to project root
cargo run --manifest-path=backend/showcase/Cargo.toml --bin=simple
```

Attack optimizer (release + rayon):

```bash
# cd to project root
cargo run --manifest-path=backend/showcase/Cargo.toml --bin=attack-optimizer --release --features=rayon
```

Attack optimizer (flamegraph):

```bash
# cd to project root
CARGO_PROFILE_RELEASE_DEBUG=true cargo flamegraph --manifest-path=backend/showcase/Cargo.toml --bin=attack-optimizer
```

Math: nearest point on arc (debug):

```bash
# cd to project root
cargo run --manifest-path=backend/showcase/Cargo.toml --bin=nearest-point-on-arc
```
