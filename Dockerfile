FROM debian:stable-slim
ARG HOST_UID=1000
ARG HOST_GID=1000
ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get install -y --no-install-recommends \
    # Build Project
    make \
    # Rust
    rustc
    && rm -rf /var/lib/apt/lists/*
RUN groupadd -g "${HOST_GID}" builder && \
    useradd -m -u "${HOST_UID}" -g "${HOST_GID}" -s /bin/bash builder
USER builder
WORKDIR /build