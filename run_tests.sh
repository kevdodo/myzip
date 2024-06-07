#!/bin/bash
cargo build --release  --bin simple_compress

time ./target/release/simple_compress test.txt

cargo build --release  --bin compress_miniz_oxide

time ./target/release/compress_miniz_oxide test.txt
