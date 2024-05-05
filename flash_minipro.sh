#!/bin/bash

set -e

# Check args
if [[ $# -ne 2 ]]; then
	echo "USAGE: $0 <JEDEC FILE> <CHIP>"
	exit 1
fi

# Program GAL
minipro -p "$2" -w "$1"

# Read GAL
READBACK=/tmp/gal_tmp
minipro -p "$2" -r "$READBACK.jed"

# Fix JEDEC for jedutil
truncate -s -7 "$READBACK.jed"
printf "*\r\n\x03" >> "$READBACK.jed"
od -t u1 -An -w1 -v "$READBACK.jed" | awk '{s+=$1; if(s > 65535) s = and(65535, s)} END {printf("%04X\r\n", s)}' >> "$READBACK.jed"

# Convert to binary and compare
jedutil -convert "$READBACK.jed" "$READBACK.bin"
jedutil -convert "$1" "${1%.jed}.bin"

if cmp -s "$READBACK.bin" "${1%.jed}.bin"; then
	echo "VERIFICATION OK!"
else
	echo "VERIFICATION FAILED!"
fi

# Clean up
rm "$READBACK.jed" "$READBACK.bin" "${1%.jed}.bin"
