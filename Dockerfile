FROM rust:1.80 as builder

WORKDIR /app

# Copy source code
COPY Cargo.toml ./
COPY src/ src/

# Remove benchmark configuration from Cargo.toml for Docker build
RUN sed -i '/\[\[bench\]\]/,$d' Cargo.toml

# Build the application
RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/zls /usr/local/bin/zls

ENTRYPOINT ["zls"]