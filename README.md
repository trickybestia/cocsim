# cocsim

Clash of Clans attack simulation project.

This material is unofficial and is not endorsed by Supercell. For more information see Supercell's Fan Content Policy: [www.supercell.com/fan-content-policy](https://www.supercell.com/fan-content-policy).

## Deploying using docker-compose

1. Building and export container image:

```bash
docker build -t cocsim/cocsim-webserver .
docker save cocsim/cocsim-webserver | zstd > cocsim-webserver.tar.zst
```

2. Transfer built container image (`cocsim-webserver.tar.zst`) to server.

3.

```bash
# on server
docker image load < cocsim-webserver.tar.zst
```

4. Transfer [docker-compose.yml](./docker-compose.yml) to server.

5. If you want to have persistent .onion address, place hidden service keys on server like this:

```
/var/lib/cocsim-webserver/tor-keys/
├── hostname
├── hs_ed25519_public_key
└── hs_ed25519_secret_key
```

6.

```bash
# on server
docker compose up -d
```

## License

Licensed under either of

- Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license
  ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
