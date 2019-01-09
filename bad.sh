#!/bin/bash

function bail
{
	echo "$1"
	exit 2
}

cargo build || bail "build failed"
readelf --wide --debug-dump=rawline target/debug/ta-client > /dev/null 2>&1 || bail "still broken"

echo "appears to be good"
