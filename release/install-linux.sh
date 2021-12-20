#!/bin/bash

data="&&DATA&&"

# Make sure user is root
if [ "$(id -u)" != "0" ]; then
	echo "This script must be run as root" 1>&2
	exit 1
fi

# Extract data to /usr/bin/cesil
echo "Extracting data..."
echo "$data" | base64 -d > /usr/bin/cesil
chmod a+rx /usr/bin/cesil

echo "Done!"
