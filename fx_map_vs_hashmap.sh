#!/bin/bash
set -euxo pipefail
make clean

cargo build --release --bin simple_compress
num_runs=30
echo

# for ((i=1; i<=num_runs; i++))
# do
#     echo "Run $i" >> times4_fx_map.txt

#     (time ./target/release/compress_miniz_oxide test32.txt) & >> times32_miniz.txt
# done

for ((i=1; i<=num_runs; i++))
do

    # Run simple_compress and save times
    (time ./target/release/simple_compress test32.txt) & >> times32_miniz.txt

    # echo "gzip and gunzip Run $i" >> times32_gzip_gunzip.txt
    # (time sh -c 'gzip test32.txt && gzip -d test32.txt.gz') 2>> times32_gzip_gunzip.txt
done
