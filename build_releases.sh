#!/bin/bash
# Builds the linux and windows releases and moves the files into the output folder

cargo fmt

cargo build --release --target x86_64-unknown-linux-gnu
cargo build --release --target x86_64-pc-windows-gnu

cp target/x86_64-unknown-linux-gnu/release/aocf-boxes-2     output/aocf-boxes-2
cp target/x86_64-pc-windows-gnu/release/aocf-boxes-2.exe     output/aocf-boxes-2.exe