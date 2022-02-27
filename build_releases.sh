#!/bin/sh
# Builds the linux and windows releases

cargo build --release --target x86_64-unknown-linux-gnu
cargo build --release --target x86_64-pc-windows-gnu

cp target/x86_64-unknown-linux-gnu/release/aocf-boxes-2      aocf-boxes-2
cp target/x86_64-pc-windows-gnu/release/aocf-boxes-2.exe     aocf-boxes-2.exe
