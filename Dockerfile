# build frontend (output file: /usr/src/frontend/dist/index.html)
FROM node:alpine3.22 AS frontend-builder
WORKDIR /usr/src/frontend
COPY ./frontend .
RUN yarn && yarn build

# build backend (output file: /usr/local/cargo/bin/webserver)
FROM rust:1.89.0-alpine3.22 AS backend-builder
WORKDIR /usr/src/backend
RUN apk add --no-cache musl-dev
COPY ./backend .
COPY --from=frontend-builder /usr/src/frontend/dist/index.html ./webserver/index.html
RUN cd webserver && cargo install --features=publish --path . && strip /usr/local/cargo/bin/webserver

FROM alpine:3.22.1
RUN apk add --no-cache tor
RUN mkdir /tor-keys
COPY ./docker/torrc /etc/tor/torrc
COPY --from=backend-builder /usr/local/cargo/bin/webserver /usr/local/bin/webserver
COPY ./docker/container-main.sh /usr/local/bin/container-main.sh
COPY ./test_maps /usr/local/share/cocsim_test_maps
ENTRYPOINT ["/usr/local/bin/container-main.sh"]
