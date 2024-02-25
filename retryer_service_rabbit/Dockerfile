# syntax=docker/dockerfile:1.3
FROM rust:1.72-bullseye as rust-build

WORKDIR /root

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    cargo install cargo-strip

COPY . .

RUN --mount=type=cache,target=/usr/local/cargo/registry --mount=type=cache,target=/root/target \
    cargo build --release && \
    cargo strip && \
    mv /root/target/release/retryer-service /root

FROM gcr.io/distroless/cc

COPY --from=rust-build /root/retryer-service /
COPY --from=rust-build /root/.env /.env

WORKDIR /
ENV RUST_LOG=info
CMD ["./retryer-service"]

EXPOSE 8080