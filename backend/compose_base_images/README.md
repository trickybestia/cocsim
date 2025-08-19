# compose_base_images

Creates composed and deskewed image of Clash of Clans base from multiple screenshots.

Running in release mode:

```bash
# cd to project root
cargo run --manifest-path=backend/compose_base_images/Cargo.toml --features=cli --release -- --images test_images/single_player/goblin_gauntlet/ --composed composed.jpg --reversed reversed.jpg
```
