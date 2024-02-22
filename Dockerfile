# Build stage
FROM rust:1-bookworm as builder
WORKDIR /app
ADD . /app
RUN cargo build --release

# Prod stage
FROM debian:bookworm-slim as runtime
COPY --from=builder /app/target/release/rustapi /

# Copy static folder
COPY --from=builder /app/static /static

CMD ["./rustapi"]
