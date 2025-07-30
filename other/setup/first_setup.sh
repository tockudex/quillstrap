#!/bin/bash

# Run from the top of the quillstrap repo

if ! command -v ~/.cargo/bin/cargo &> /dev/null || \
   ! command -v ~/.cargo/bin/rustup &> /dev/null || \
   ! command -v ~/.cargo/bin/cbindgen &> /dev/null; then
       
    echo "Installing Rust"
    curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain stable -y
    
    source ~/.cargo/env
    export PATH="${HOME}/.cargo/bin:${PATH}"

    ~/.cargo/bin/cargo install --force cbindgen
else
    echo "Rust is already installed, skipping installation."
fi