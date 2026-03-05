FROM rust:slim-bookworm

ARG ARM_GNU_TOOLCHAIN_VERSION=15.2.rel1

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
# Use the shared helper script so local and Docker installs use the same flow
COPY scripts/install_arm_gcc.sh /tmp/install_arm_gcc.sh
RUN bash /tmp/install_arm_gcc.sh --version ${ARM_GNU_TOOLCHAIN_VERSION} --install-dir /opt/gcc-arm-none-eabi \
    && rm -f /tmp/install_arm_gcc.sh

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

# Copy the workspace into the container (this happens at build time)
COPY . /workspace
