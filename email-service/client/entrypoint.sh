#!/bin/sh

cd client || exit 1

while true; do
	diesel migration run && break
	echo "Database upgrade failed, retrying in 5 seconds..."
	sleep 5
done

exec /usr/local/bin/launcher client 2>&1
