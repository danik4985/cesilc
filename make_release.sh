#!/bin/bash

echo "Time to make a release"

[[ ! -d "release" ]] && mkdir release

function addtarget {
	echo -e "\e[2;3mWill add $1 if it is not installed.\e[0m"

	(rustup target list --installed | grep -q "$1") || rustup target add "$1"
}

for i in x86_64-unknown-linux-gnu x86_64-apple-darwin x86_64-pc-windows-gnu; do
	addtarget "$i"

	echo -e "\e[2mBuilding for $i\e[0m"
	cargo build --release --target "$i" 2>/dev/null

	res=$?

	if [[ $res -ne 0 ]]; then
		echo -e "\e[31mFailed to build for $i with code $res\e[0m"
	fi

	if [[ $res -eq 0 ]]; then
		echo -e "\e[32mBuild successful for $i\e[0m"
		cp target/$i/release/cesilc* release/
	fi
done

rm -f "release/cesilc.d"

echo -e "\e[1mTurning exe file into a header\e[0m"
xxd -i release/cesilc.exe > release/cesilc.h

echo -e "\e[1mCreating install script for linux\e[0m"
node gen-install-linux.js

echo -e "\e[1mCreating installer for shitdows\e[0m"
i686-w64-mingw32-gcc release/install-shitdows.c -o install-shitdows.exe

echo -e "\e[32;1mDone in ${SECONDS}s!\e[0m"
