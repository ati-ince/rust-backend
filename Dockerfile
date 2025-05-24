# Build stage
FROM rust:latest as builder
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim
COPY --from=builder /usr/src/app/target/release/rust-backend /usr/local/bin/rust-backend
EXPOSE 8000
CMD ["rust-backend"] 