FROM rust:slim
ARG HOST_UID=1000
ARG HOST_GID=1000
RUN apt-get update && apt-get install -y --no-install-recommends \
    # Build Project
    make \
    && rm -rf /var/lib/apt/lists/*
RUN rustup target add \
    # Musl
    x86_64-unknown-linux-musl \
    aarch64-unknown-linux-musl \
    riscv64gc-unknown-linux-musl \
    # None
    x86_64-unknown-none \
    aarch64-unknown-none \
    riscv64gc-unknown-none-elf
RUN groupadd -g "${HOST_GID}" builder && \
    useradd -m -u "${HOST_UID}" -g "${HOST_GID}" -s /bin/bash builder
USER builder
WORKDIR /build