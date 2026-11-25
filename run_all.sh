#!/usr/bin/env bash
set -euo pipefail

start=$(date +%s%N)

for num in {1..25}; do
    cargo run --release --quiet -- "$num" >/dev/null 2>&1
done

end=$(date +%s%N)
elapsed_ns=$((end - start))

printf "[%d.%06ds] :total\n" \
    $((elapsed_ns / 1000000000)) \
    $(((elapsed_ns / 1000) % 1000000))
