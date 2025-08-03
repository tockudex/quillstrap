FROM --platform=linux/amd64 ubuntu:24.04

# Missing stuff in container
# For some reason, apt update needs to be with the first install via &&
RUN apt-get update && apt-get install -y htop psmisc

# Rust stuff
RUN apt-get install -y gcc build-essential curl pkg-config libinput10
ENV PATH="${PATH}:/root/.cargo/bin"

# VS Code stuff
RUN apt-get install -y git

# quillstrap stuff
RUN apt-get install -y openssl libssl-dev clang

# U-Boot build
RUN apt-get install -y coreutils device-tree-compiler python3 python3-pyelftools gcc-aarch64-linux-gnu