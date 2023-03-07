# Use the Rust base image
FROM rust:latest

# Create ENV dirpath
ENV DIRPATH /Users/jasonzhang/Documents/cyber/Project/rust_proj

# Create a new directory for the project
WORKDIR $DIRPATH/socket_programming

RUN pwd

# Copy the entire project directory into the container
COPY . .

# Build the Rust project inside the container
RUN cargo build --release

# Set the entry point to run the Rust binary
CMD ["./target/release/myproject"]
