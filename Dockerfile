# Stage 1: Build the Rust project using the official Rust image
FROM rust:1.82-slim-bullseye AS build

# Set the working directory inside the container
WORKDIR /money-way

# Install system dependencies for OpenSSL and pkg-config
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*


# Copy the entire project source code into the container
COPY . .

# Compile the Rust project in release mode (optimized build)
RUN cargo build --release


# Stage 2: Create a minimal final image with only the compiled binary
# FROM gcr.io/distroless/cc
FROM debian:bullseye-slim

# Set the working directory in the final image
WORKDIR /money-way

# Copy the compiled binary from the build stage into this minimal image
COPY --from=build /money-way/target/release/money-way .

# Inform that the container will listen on port 80 (useful for documentation and orchestration tools)
EXPOSE 80

# Define the default command to run the application when the container starts
CMD ["./money-way"]