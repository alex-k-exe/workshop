# ---- Build Stage ----
FROM rust:1.70.0-slim-bullseye AS builder
WORKDIR /app

# Cache dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# Build actual project
COPY . .
RUN cargo build --release

# ---- Runtime Stage ----
FROM debian:bullseye-slim

# Create a non-root user (optional, for security)
RUN adduser --disabled-password --gecos "" appuser

WORKDIR /app
COPY --from=builder /app/target/release/workshop /app/
USER appuser

CMD ["./workshop"]
