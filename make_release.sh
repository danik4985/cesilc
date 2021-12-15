#!/bin/bash

echo "Time to make a release"

status=""

[[ ! -d "release" ]] && mkdir release

function addtarget {
	echo "Will add $1 if it is not installed."

	(rustup target list --installed | grep -q "$1") || rustup target add "$1"
}

for i in x86_64-unknown-linux-gnu x86_64-apple-darwin x86_64-pc-windows-gnu; do
	addtarget "$i"

	echo "Building for $i"
	cargo build --release --target "$i" 2>/dev/null

	res=$?

status="$status
$i : $res"

	if [[ $res -ne 0 ]]; then
		echo "Failed to build for $i"
	fi

	if [[ $res -eq 0 ]]; then
		echo "Copying for $i"
		cp target/$i/release/cesilc* release/
	fi
done

rm -f "release/cesilc.d"

echo
echo -e "\e[1m$status\e[0m"
