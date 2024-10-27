FROM rust:latest as builder
WORKDRI /build
COPY . .
RUN cargo build --release

FROM rust:latest
WORKDIR /app
COPY --from=builder /build/target/release/rust-cloud-run .
CMD ["./rust-cloud-run"]