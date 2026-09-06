#!/usr/bin/env bash
# Scaffold a crate for a Rosalind problem: bin/new-problem.sh <id>
set -euo pipefail

id="${1:?usage: new-problem.sh <id>}"
id="$(echo "$id" | tr '[:upper:]' '[:lower:]')"
root="$(cd "$(dirname "$0")/.." && pwd)"
crate="$root/crates/$id"

if [ -e "$crate" ]; then
  echo "error: crates/$id already exists" >&2
  exit 1
fi

mkdir -p "$crate/src"
sed "s/{ID}/$id/g" "$root/template/Cargo.toml" > "$crate/Cargo.toml"
sed "s/{ID}/$id/g" "$root/template/src/main.rs" > "$crate/src/main.rs"
mkdir -p "$root/data/$id"
touch "$root/data/$id/input.txt"

echo "created crates/$id and data/$id"
echo "next: download the dataset into data/$id/input.txt, implement src/main.rs with inline sample tests"
