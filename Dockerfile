# Use the official Rust image as the base image
FROM rust:latest as builder

# Create a new empty shell project
RUN USER=root cargo new --bin grpc-server
WORKDIR /grpc-server

# Copy your project's Cargo.toml and Cargo.lock and your src directory
COPY ./Cargo.toml ./Cargo.lock ./
COPY ./src ./src

RUN cargo build --release

# Use the Debian Buster image for the final build to reduce size
FROM debian:buster-slim
COPY --from=builder /grpc-server/target/release/grpc-server /usr/local/bin/grpc-server

# Expose
EXPOSE 50051

# Command to run the executable
CMD ["grpc-server"]

