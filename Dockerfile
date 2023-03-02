# Use a base image with Rust and the required libraries installed
FROM rust:latest

# Set the working directory in the container
WORKDIR /app

# Copy the source code to the container
COPY . .

# Build the application in release mode
RUN cargo build --release

# Set the environment variables for the application
ENV ROCKET_ENV=prod

# Run the application
CMD ["./target/release/rust_api"]
