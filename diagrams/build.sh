#!/usr/bin/env bash
# Builds every diagram into static/diagrams/.
#
# Run this locally (or in CI) and commit the output: Cloudflare Pages does not
# need a Rust toolchain, matching how the Cocos game bundles are already
# shipped. build.sh at the repo root deliberately does not call this.
set -euo pipefail

here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
root="$(dirname "$here")"
out="$root/static/diagrams"

# 1. Static SVGs + the accessible table, from the shared core.
cargo run --release --manifest-path "$here/Cargo.toml" -p diagrams-svg -- "$out"

# 2. The same core, compiled for the browser. One module, all three diagrams.
cargo build --release --target wasm32-unknown-unknown \
  --manifest-path "$here/Cargo.toml" -p diagrams-wasm

wasm-bindgen --target web --no-typescript --out-dir "$out" \
  "$here/target/wasm32-unknown-unknown/release/diagrams_wasm.wasm"

if command -v wasm-opt >/dev/null 2>&1; then
  wasm-opt -Oz --enable-bulk-memory --enable-nontrapping-float-to-int \
    -o "$out/diagrams_wasm_bg.wasm" "$out/diagrams_wasm_bg.wasm"
else
  echo "note: wasm-opt not found, shipping unoptimised wasm" >&2
fi

printf '\n%-34s %8s\n' "artifact" "bytes"
find "$out" -type f | sort | while read -r f; do
  printf '%-34s %8s\n' "${f#"$out"/}" "$(wc -c < "$f")"
done
