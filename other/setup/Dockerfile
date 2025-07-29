FROM --platform=linux/amd64 ubuntu:24.04

# Rust stuff
RUN apt-get update && apt-get install -y gcc build-essential curl pkg-config libinput10
ENV PATH=$PATH:/root/.cargo/bin

# Vscode stuff
RUN apt-get install -y git