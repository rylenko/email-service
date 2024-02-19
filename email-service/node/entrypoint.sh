#!/bin/sh

cd node/

while true; do
	diesel migration run
	if [ "$?" -eq "0" ]; then
		break
	fi
	echo "Database upgrade failed, retrying in 5 seconds..."
	sleep 5
done

exec /usr/local/bin/launcher node
