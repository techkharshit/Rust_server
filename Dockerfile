# Stage 1: Build the application using a Rust Alpine image
FROM rust:alpine as builder

# Install required build dependencies
RUN apk add --no-cache musl-dev

# Create a new directory for the project
WORKDIR /usr/src/app

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Copy the source code
COPY src ./src

# Build the application in release mode
RUN cargo build --release

# Stage 2: Create the final image with the compiled binary
FROM alpine:latest

# Install necessary packages
RUN apk add --no-cache ca-certificates

# Create a directory for the application
WORKDIR /app

# Copy the compiled binary from the build stage
COPY --from=builder /usr/src/app/target/release/rust /usr/local/bin/rust

# Copy the test.txt file into the image
COPY test.txt /app/test.txt

# Set the binary as the entry point
ENTRYPOINT ["/usr/local/bin/rust"]

# Expose the port that the app runs on
EXPOSE 8080
