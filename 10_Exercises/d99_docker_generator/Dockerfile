FROM rust:1.74 AS builder
        WORKDIR /usr/src/d99_docker_generator
        COPY . .
        RUN cargo build --release
        
        FROM debian:bookworm-slim
        WORKDIR /app
        COPY --from=builder /usr/src/d99_docker_generator/target/release/d99_docker_generator /app/d99_docker_generator
        CMD ["./d99_docker_generator"]
        