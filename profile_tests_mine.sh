#!/bin/bash

set -euxo pipefail

make clean

cargo build --bin simple_compress

valgrind --tool=callgrind --callgrind-out-file=callgrinds/callgrind.out.%p target/debug/simple_compress test_short.txt

# Find the latest callgrind output file
latest=$(ls -t callgrinds/callgrind.out.* | head -n 1)
export $(dbus-launch)
# Open the latest output file with KCachegrind
kcachegrind $latest
# cargo build --release  --bin compress_miniz_oxide

# time ./target/release/compress_miniz_oxide test.txt
