# Use the Rust official image for building
FROM rust:1.84 as builder

# Set the working directory inside the container
WORKDIR /usr/src/app

COPY . .
RUN cargo build --release

# Use a minimal runtime image
FROM debian:bookworm

# Install dependencies
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Set the working directory
WORKDIR /app

# Copy the binary from the builder stage
COPY --from=builder /usr/src/app/target/release/github_notification_pusher /app/github_notification_pusher

# Expose the application port
EXPOSE 8080

# Run the application
CMD ["./github_notification_pusher"]
