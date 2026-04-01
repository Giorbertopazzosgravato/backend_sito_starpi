# --- Stage 1: The Builder ---
# Use the official Rust image to compile the application
FROM rust:1.94 as builder

# Create a working directory inside the container
WORKDIR /usr/src/app

# Copy your entire project into the container
COPY . .

# Compile the application in release mode
RUN cargo build --release

# --- Stage 2: The Runtime Environment ---
# Use a minimal Debian image for the final runtime
FROM debian:bookworm-slim

# (Optional) Install SSL certificates if your app makes external HTTPS requests
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app
# Copy the compiled binary from the 'builder' stage into this new, clean stage
# IMPORTANT: Replace 'my_rust_project' with the actual name of your app from your Cargo.toml
COPY --from=builder /usr/src/app/target/release/test_server_starpi_nolibs /usr/local/bin/test_server_starpi_nolibs

# Set the command to run your binary
CMD ["test_server_starpi_nolibs"]