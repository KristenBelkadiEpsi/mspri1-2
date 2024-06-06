
FROM rust:latest


RUN mkdir /app
WORKDIR /app


COPY .. .


RUN cargo build --release


EXPOSE 8080

# Démarrez votre application
CMD ["./target/release/your-app-name"]
