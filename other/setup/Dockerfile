FROM --platform=linux/amd64 ubuntu:24.04

# Missing stuff in container
# For some reason, apt update needs to be with the first install via &&
RUN apt-get update && apt-get install -y htop psmisc wget unzip file

# Rust stuff
RUN apt-get install -y gcc build-essential curl pkg-config libinput10 pkg-config libusb-1.0-0-dev libftdi1-dev libudev-dev
ENV PATH="${PATH}:/root/.cargo/bin"

# VS Code stuff
RUN apt-get install -y git

# quillstrap stuff
RUN apt-get install -y openssl libssl-dev clang

# U-boot build
RUN apt-get install -y coreutils device-tree-compiler python3 python3-pyelftools gcc-aarch64-linux-gnu 

## U-boot boot menu
RUN apt-get install -y python3-pillow fonts-dejavu-core fonts-freefont-ttf fontconfig imagemagick libpng-dev cmake
### Instal u001 font
RUN mkdir -p /tmp/fonti && \
    cd /tmp/fonti && \
    wget https://fontlibrary.org/assets/downloads/u001/3ea00b3c0c8fa6ce4373e5766fafd651/u001.zip && \
    unzip u001.zip && \
    mkdir -p /usr/share/fonts/u001 && \
    cp *.ttf /usr/share/fonts/u001/ && \
    fc-cache -fv && \
    rm -rf /tmp/fonti

# Build & install rkdeveloptool (From here, because docker)
RUN apt-get install -y meson libusb-1.0-0-dev udev
RUN cd /tmp && \
    git clone https://github.com/PorQ-Pine/rkdeveloptool.git && \
    cd rkdeveloptool && \
    meson build && \
    meson compile -C build && \
    meson install -C build && \
    rm -rf /tmp/rkdeveloptool
