#!/usr/bin/env bash
# Generate sample data, then stream-parse it in both formats and show stats.
set -euo pipefail

cargo build --release
mkdir -p sample-data

echo "== Generating sample NDJSON =="
{
    for i in $(seq 1 5); do
        echo "{\"id\": $i, \"name\": \"item-$i\"}"
    done
} > sample-data/items.ndjson

echo "== Generating sample delimited =="
{
    echo "id, name, qty"
    for i in $(seq 1 5); do
        echo "$i, item-$i, $((i * 10))"
    done
} > sample-data/items.csv

echo
echo "== Parse NDJSON =="
./target/release/streamparse parse --format ndjson sample-data/items.ndjson

echo
echo "== Parse delimited =="
./target/release/streamparse parse --format delimited sample-data/items.csv

echo
echo "== Stats (NDJSON) =="
./target/release/streamparse stats --format ndjson sample-data/items.ndjson
