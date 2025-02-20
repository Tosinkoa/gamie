FROM rust:1.81 as builder
WORKDIR /app

# Copy manifest and source, then build dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir src
RUN echo "fn main() { println!(\"Dummy main\"); }" > src/main.rs

# Build release binary
RUN cargo build --release

# Copy source code and rebuild
COPY . .
RUN cargo build --release

FROM debian:buster-slim

# Copy the compiled binary from builder
COPY --from=builder /app/target/release/backend /usr/local/bin/backend

EXPOSE 8000

CMD ["backend"] 