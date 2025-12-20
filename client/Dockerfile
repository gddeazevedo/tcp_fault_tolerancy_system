# Use a Rust base image with Cargo installed
FROM rust:1.88.0 AS builder

# Set the working directory inside the container
WORKDIR /usr/src/app

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Now copy the source code
COPY ./src ./src

# Command to run the application
CMD ["cargo", "run", "12"]