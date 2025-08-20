#!/bin/bash

/usr/bin/tor &

sleep 1

cat /var/lib/tor/cocsim-webserver/hostname

TEST_MAPS_PATH=/usr/local/share/cocsim_test_maps RUST_LOG=tower_http=trace,webserver=debug RUST_BACKTRACE=1 /usr/local/bin/webserver &

/bin/bash
