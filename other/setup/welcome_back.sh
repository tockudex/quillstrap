#!/bin/bash
if [ -e "/sys/module/rosetta" ]; then
    CARGO_DIR="root/.cargo"
    RUSTUP_DIR="root/.rustup"
    if ! mountpoint "${CARGO_DIR}"; then
        mkdir -p "${CARGO_DIR}" "/${CARGO_DIR}"
        mount --bind "${CARGO_DIR}" "/${CARGO_DIR}"
    fi
    if ! mountpoint "${RUSTUP_DIR}"; then
        mkdir -p "${RUSTUP_DIR}" "/${RUSTUP_DIR}"
        mount --bind "${RUSTUP_DIR}" "/${RUSTUP_DIR}"
    fi
fi
git pull
rq -m -g all
