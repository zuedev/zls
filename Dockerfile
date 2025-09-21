FROM rust:1.75 as builder

WORKDIR /app

# Copy manifests
COPY Cargo.toml ./

# Create a dummy main.rs to build dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build dependencies (this will be cached if Cargo.toml doesn't change)
RUN cargo build --release && rm -rf src

# Copy source code
COPY src/ src/

# Build the actual application
RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/zls /usr/local/bin/zls

ENTRYPOINT ["zls"]