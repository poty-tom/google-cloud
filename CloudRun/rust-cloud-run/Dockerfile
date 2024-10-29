FROM rust:latest as builder
WORKDIR /build
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /build/target/release/rust-cloud-run .
EXPOSE 8080
CMD ["./rust-cloud-run"]