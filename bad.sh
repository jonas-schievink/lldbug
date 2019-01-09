#!/bin/bash

function bail
{
	echo "$1"
	exit 2
}

export RUSTFLAGS=-Ccodegen-units=1
export CARGO_INCREMENTAL=0
cargo build || bail "build failed"
output=$(readelf --wide --debug-dump=rawline target/debug/ta-client 2>&1 > /dev/null)

echo "$output"
echo "$output" | grep "the section is too small" > /dev/null || bail "!!! appears to be FIXED !!!"

echo "still broken - good"
exit 0
