FROM rust:latest as builder
WORKDIR /build
COPY . .
RUN cargo build --release

FROM rust:latest
WORKDIR /app
COPY --from=builder /build/target/release/rust-app-engine .
CMD ["./rust-app-engine"]