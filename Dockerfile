FROM rust:slim-bookworm

# Install required tools
# - make: for running makefiles in tests
# - git: for cloning lilos and other dependencies
# - wget, xz-utils: for downloading and extracting ARM toolchain
RUN apt-get update && apt-get install -y \
    make \
    git \
    wget \
    xz-utils \
    && rm -rf /var/lib/apt/lists/*

# Install ARM GNU Toolchain
# We download a specific version to ensure reproducible builds and compatibility with tests
RUN wget -q https://developer.arm.com/-/media/Files/downloads/gnu/13.2.rel1/binrel/arm-gnu-toolchain-13.2.rel1-x86_64-arm-none-eabi.tar.xz -O /tmp/arm-toolchain.tar.xz \
    && mkdir -p /opt/gcc-arm-none-eabi \
    && tar -xf /tmp/arm-toolchain.tar.xz --strip-components=1 -C /opt/gcc-arm-none-eabi \
    && rm /tmp/arm-toolchain.tar.xz

# Add rust targets needed for tests
RUN rustup target add \
    thumbv6m-none-eabi \
    thumbv7m-none-eabi \
    thumbv7em-none-eabi \
    thumbv7em-none-eabihf

# Set environment variable expected by test_gcc.sh
ENV GCC_HOME=/opt/gcc-arm-none-eabi
ENV PATH="${GCC_HOME}/bin:${PATH}"

# Configure git to trust all directories
# This is necessary because we are copying the git repository and running as root
RUN git config --global --add safe.directory '*'

WORKDIR /workspace

ENV CARGO_TARGET_DIR=/workspace/target

# Copy the workspace into the container (this happens at build time)
COPY . /workspace
