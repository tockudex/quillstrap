#!/bin/bash

# Run from the top of the quillstrap repo

if ! command -v ~/.cargo/bin/cargo &> /dev/null || \
   ! command -v ~/.cargo/bin/rustup &> /dev/null || \
   ! command -v ~/.cargo/bin/rmodem &> /dev/null || \
   ! command -v ~/.cargo/bin/cbindgen &> /dev/null; then
       
    echo "Installing Rust"
    curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain stable -y
    
    source ~/.cargo/env
    export PATH="${HOME}/.cargo/bin:${PATH}"

    ~/.cargo/bin/cargo install --force cbindgen

    ~/.cargo/bin/cargo install --features 'cli' --force rmodem
else
    echo "Rust is already installed, skipping installation."
fi

# Check if aarch64-unknown-linux-musl target is installed
if ! rustup target list --installed | grep -q "^aarch64-unknown-linux-musl$"; then
    echo "Installing aarch64-unknown-linux-musl target"
    rustup target add aarch64-unknown-linux-musl
else
    echo "Target aarch64-unknown-linux-musl is already installed."
fi