FROM --platform=linux/amd64 docker.io/rust:bookworm
RUN apt-get update
RUN apt-get install -y libsystemd-dev openssl
RUN rustup target add x86_64-unknown-linux-gnu
WORKDIR /usr/src/build
