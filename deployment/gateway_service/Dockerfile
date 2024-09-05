# syntax=docker/dockerfile:1.3
FROM rust:1.80-bullseye as build

WORKDIR /root

RUN apt-get update && apt-get install libpq5 -y && apt-get install libz3-dev -y

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    cargo install cargo-strip

COPY ../../dscoursework-backend .

RUN --mount=type=cache,target=/usr/local/cargo/registry --mount=type=cache,target=/root/target \
    cargo build --package gateway-service --release && \
    cargo strip && \
    mv /root/target/release/gateway-service /root

FROM rust:1.80-bullseye

ARG ARCH=x86_64

COPY --from=build /usr/lib/${ARCH}-linux-gnu/libpq.so* /usr/lib/${ARCH}-linux-gnu/
COPY --from=build /usr/lib/${ARCH}-linux-gnu/libgssapi_krb5.so* /usr/lib/${ARCH}-linux-gnu/
COPY --from=build /usr/lib/${ARCH}-linux-gnu/libldap_r-2.4.so* /usr/lib/${ARCH}-linux-gnu/
COPY --from=build /usr/lib/${ARCH}-linux-gnu/libkrb5.so* /usr/lib/${ARCH}-linux-gnu/
COPY --from=build /usr/lib/${ARCH}-linux-gnu/libk5crypto.so* /usr/lib/${ARCH}-linux-gnu/
COPY --from=build /usr/lib/${ARCH}-linux-gnu/libkrb5support.so* /usr/lib/${ARCH}-linux-gnu/
COPY --from=build /usr/lib/${ARCH}-linux-gnu/liblber-2.4.so* /usr/lib/${ARCH}-linux-gnu/
COPY --from=build /usr/lib/${ARCH}-linux-gnu/libsasl2.so* /usr/lib/${ARCH}-linux-gnu/
COPY --from=build /usr/lib/${ARCH}-linux-gnu/libgnutls.so* /usr/lib/${ARCH}-linux-gnu/
COPY --from=build /usr/lib/${ARCH}-linux-gnu/libp11-kit.so* /usr/lib/${ARCH}-linux-gnu/
COPY --from=build /usr/lib/${ARCH}-linux-gnu/libidn2.so* /usr/lib/${ARCH}-linux-gnu/
COPY --from=build /usr/lib/${ARCH}-linux-gnu/libunistring.so* /usr/lib/${ARCH}-linux-gnu/
COPY --from=build /usr/lib/${ARCH}-linux-gnu/libtasn1.so* /usr/lib/${ARCH}-linux-gnu/
COPY --from=build /usr/lib/${ARCH}-linux-gnu/libnettle.so* /usr/lib/${ARCH}-linux-gnu/
COPY --from=build /usr/lib/${ARCH}-linux-gnu/libhogweed.so* /usr/lib/${ARCH}-linux-gnu/
COPY --from=build /usr/lib/${ARCH}-linux-gnu/libgmp.so* /usr/lib/${ARCH}-linux-gnu/
COPY --from=build /usr/lib/${ARCH}-linux-gnu/libffi.so* /usr/lib/${ARCH}-linux-gnu/
COPY --from=build /usr/lib/${ARCH}-linux-gnu/libcom_err.so* /lib/${ARCH}-linux-gnu/
COPY --from=build /usr/lib/${ARCH}-linux-gnu/libkeyutils.so* /lib/${ARCH}-linux-gnu/
COPY --from=build /usr/lib/${ARCH}-linux-gnu/libz.so* /lib/${ARCH}-linux-gnu/

COPY --from=build /root/gateway-service /

WORKDIR /
ENV RUST_LOG=info
CMD ["./gateway-service"]
