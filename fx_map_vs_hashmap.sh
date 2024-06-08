#!/bin/bash
set -euxo pipefail
make clean

cargo build --release --bin simple_compress
num_runs=50
echo

for ((i=1; i<=num_runs; i++))
do
    echo "Run $i" >> times4_fx_map.txt

    # Run simple_compress and save times
    (time ./target/release/simple_compress test4.txt) &>> times4_fx_map.txt
done
