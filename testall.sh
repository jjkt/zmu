#!/bin/bash
set -e

#
# To clean the persistent volumes, run:
# docker volume rm zmu-cargo-registry zmu-target-cache zmu-rustup-home
#
#
# Build the docker image
# Use the current directory as context
echo "Building Docker image..."
docker build -t zmu-test-env .

# Run the tests inside the container
echo "Running tests in Docker container..."
docker run --rm \
    -v zmu-cargo-registry:/usr/local/cargo/registry \
    -v zmu-rustup-home:/usr/local/rustup \
    -v zmu-target-cache:/workspace/target \
    zmu-test-env ./run_tests_internal.sh

