#!/bin/bash

data="&&DATA&&"

# Make sure user is root
if [ "$(id -u)" != "0" ]; then
	# echo "This script must be run as root" 1>&2
	# exit 1
	echo "You need administrator perms to run this script"
	sudo "$0"
	exit $?
fi

echo "Installing CesilC version &&VERSION&&"

# If cesilc already installed, remove it
if [[ -f /usr/local/bin/cesilc ]]; then
	echo "Removing old cesilc binary"
	sudo rm -rf /usr/bin/cesilc
fi

# Extract data to /usr/bin/cesil
echo "Extracting data..."
echo "$data" | base64 -d > tmp.cesil.to_be_copied
sudo cp tmp.cesil.to_be_copied /usr/bin/cesilc
sudo chmod a+rx /usr/bin/cesil
sudo rm -rf tmp.cesil.to_be_copied

echo "Done!"
