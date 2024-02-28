#!/bin/bash

RUSTFLAGS="-Zlocation-detail=none" cargo build --release
RUSTFLAGS="-Zlocation-detail=none" cargo build --target=x86_64-unknown-linux-musl --release
RUSTFLAGS="-Zlocation-detail=none" cargo build --target x86_64-pc-windows-gnu --release --verbose
upx --best --lzma target/release/mydogchase
upx --best --lzma target/x86_64-unknown-linux-musl/release/mydogchase
upx --best --lzma target/x86_64-pc-windows-gnu/release/mydogchase.exe
