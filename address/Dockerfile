FROM postgres
FROM rust:1.78-bookworm as builder

WORKDIR /usr/src
COPY . .
RUN cargo build --release
RUN ls -la /usr/src/target/release

FROM debian:bookworm
WORKDIR /usr/src
COPY --from=builder /usr/src/.env /usr/src
COPY --from=builder /usr/src/target/release/address /usr/src/

