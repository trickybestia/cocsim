#!/bin/ash

# Initialize tor hidden service keys (if provided via bind mount)
cp -R /tor-keys /var/lib/tor/cocsim-webserver
chown -R tor:tor /var/lib/tor/cocsim-webserver
chmod -R 700 /var/lib/tor/cocsim-webserver

# Delete unix socket from previous container run (if exists)
rm -f /run/cocsim-webserver.sock

/usr/bin/tor &

sleep 1

cat /var/lib/tor/cocsim-webserver/hostname

TEST_MAPS_PATH=/usr/local/share/cocsim_test_maps RUST_LOG=tower_http=trace,webserver=debug RUST_BACKTRACE=1 exec /usr/local/bin/webserver
