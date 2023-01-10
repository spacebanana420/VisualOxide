#!/bin/bash
#Linux
cargo build -r --target x86_64-unknown-linux-gnu
cargo build -r --target x86_64-unknown-linux-musl
cargo build -r --target aarch64-unknown-linux-gnu
cargo build -r --target aarch64-unknown-linux-musl

#MacOS
cargo build -r --target x86_64-apple-darwin
cargo build -r --target aarch64-apple-darwin

#Windows
cargo build -r --target x86_64-pc-windows-msvc

#BSD
cargo build -r --target x86_64-unknown-freebsd
cargo build -r --target aarch64-unknown-freebsd
cargo build -r --target x86_64-unknown-openbsd
cargo build -r --target aarch64-unknown-openbsd
cargo build -r --target x86_64-unknown-netbsd
cargo build -r --target aarch64-unknown-netbsd
