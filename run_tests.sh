#!/bin/bash
set -euxo pipefail
make clean

cargo build --release  --bin simple_compress

time ./target/release/simple_compress test2.txt
time ./target/release/simple_compress test4.txt
time ./target/release/simple_compress test8.txt

# cargo build --release  --bin compress_miniz_oxide

# time ./target/release/compress_miniz_oxide test2.txt
# time ./target/release/compress_miniz_oxide test4.txt
# time ./target/release/compress_miniz_oxide test8.txt
