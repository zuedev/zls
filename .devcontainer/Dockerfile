FROM mcr.microsoft.com/vscode/devcontainers/rust:1-bookworm

# Install additional tools
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends \
        git \
        curl \
        build-essential \
        pkg-config \
        libssl-dev \
    && apt-get autoremove -y && apt-get clean -y && rm -rf /var/lib/apt/lists/*

# Set the default user
USER vscode

# Install additional Rust tools
RUN rustup component add clippy rustfmt

# Set working directory
WORKDIR /workspace