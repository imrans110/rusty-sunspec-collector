#!/bin/bash -eu
# Builds the fuzz targets and their seed corpora for ClusterFuzzLite. The OSS-Fuzz image's
# cargo adds the sanitizer flags.

targets="decode_model model_chain config"

cd "$SRC/rusty-sunspec-collector"
cargo fuzz build -O
for target in $targets; do
  cp "fuzz/target/x86_64-unknown-linux-gnu/release/$target" "$OUT/"
done

python3 fuzz/seeds.py "$WORK/seeds"
for target in $targets; do
  (cd "$WORK/seeds/$target" && zip -q -r "$OUT/${target}_seed_corpus.zip" .)
done
