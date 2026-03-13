# Use the official Rust image as the build stage
FROM rust:1.94.0-alpine3.23 as builder

# Set the working directory inside the container
WORKDIR /usr/src/app

# Copy the source code to the working directory
COPY . .

# Build the Rust application
RUN cargo build --package todo-api --bin get-todos --release

# Use a smaller base image for the final container
FROM alpine:3.21

# Copy the compiled binary from the build stage
COPY --from=builder /usr/src/app/target/release/get-todos /usr/local/bin/call-api

# Set the entrypoint command
CMD ["call-api"]