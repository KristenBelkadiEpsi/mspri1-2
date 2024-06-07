FROM rust:1.78-bookworm as builder
COPY . .
CMD ["curl", "http://localhost:2345"]
CMD ["cargo", "run", "--release"]
