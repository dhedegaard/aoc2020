FROM rust:1 AS rust

WORKDIR /aoc2020

# Install wasm-pack.
RUN cargo install wasm-pack

# Install dependencies.
COPY aoc2020/Cargo.toml aoc2020/Cargo.lock ./
RUN ls -lha .
RUN wasm-pack build
RUN cargo fetch --frozen

# Run tests.
COPY aoc2020/* ./
RUN cargo test

# Build the wasm output
RUN wasm-pack build
