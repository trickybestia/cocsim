# build frontend (output file: /usr/src/frontend/dist/index.html)
FROM node:alpine3.22 AS frontend-builder
WORKDIR /usr/src/frontend
COPY ./frontend .
RUN yarn && yarn build

# build backend (output file: /usr/local/cargo/bin/webserver)
FROM rust:1.89.0 AS backend-builder
WORKDIR /usr/src/backend
RUN apt update -y && apt install --no-install-recommends -y libclang-dev
COPY ./backend .
COPY --from=frontend-builder /usr/src/frontend/dist/index.html ./webserver/index.html
RUN cd webserver && cargo install --features=publish --path .

FROM debian:trixie-slim
RUN apt update -y && apt install --no-install-recommends -y imagemagick tor
COPY ./resources/torrc /etc/tor/torrc
COPY --from=backend-builder /usr/local/cargo/bin/webserver /usr/local/bin/webserver
USER debian-tor
ENTRYPOINT ["/usr/bin/bash"]
