#!/bin/bash

# Exit immediately if a command exits with a non-zero status
set -e

# Function to run Podman command
run_podman() {
    podman run -tiv "$(pwd)":/usr/src/build:Z \
        -v cargo-git:/home/rust/.cargo/git \
        -v cargo-registry:/home/rust/.cargo/registry \
        rust-builder cargo "$@"
}

run_podman "$@"
