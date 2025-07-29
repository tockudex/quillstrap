FROM --platform=linux/amd64 ubuntu:24.04

# Container missing stuff
# For some reason, apt update needs to be with the first install via &&
RUN apt-get update && apt-get install -y htop psmisc

# Rust stuff
RUN apt-get install -y gcc build-essential curl pkg-config libinput10
ENV PATH=$PATH:/root/.cargo/bin

# Vscode stuff
RUN apt-get install -y git

# Quillstrap stuff
RUN apt-get install -y openssl libssl-dev clang