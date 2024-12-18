#!/usr/bin/env bash
echo "It should build binaries in dist/<platform>-<arch>[.exe] as needed."
cargo build --release --target=$TARGET 
mkdir -p dist
cp target/$TARGET/release/gh-ghpm-rs dist/$DIST_FILE_NAME
