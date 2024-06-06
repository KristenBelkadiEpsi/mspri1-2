FROM rust:1.78.0-buster as builder


RUN mkdir /app


COPY . .


RUN cargo build --release


EXPOSE 8080

# Démarrez votre application
CMD ["./target/release/address"]
