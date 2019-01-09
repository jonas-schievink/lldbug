#!/bin/bash

function bail
{
	echo "$1"
	exit 2
}

export RUSTFLAGS=-Ccodegen-units=1
export CARGO_INCREMENTAL=0
cargo build $@ || bail "build failed"
output=$(dwarf-validate target/debug/ta-client)

echo "$output"
echo "$output" | grep "DWARF error" > /dev/null || bail "!!! appears to be FIXED !!!"

echo "still broken - good"
exit 0
