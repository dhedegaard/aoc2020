FROM rust:1 AS rust

WORKDIR /aoc2020

# Install wasm-pack.
RUN cargo install wasm-pack

# Install dependencies.
COPY aoc2020/  ./
RUN cargo fetch --locked

# Run tests.
RUN cargo test

# Build the wasm output
RUN wasm-pack build

FROM node:14-alpine

# Install dependencies
WORKDIR /web
COPY web/package.json web/yarn.lock ./
RUN yarn

# Copy over the WASM build.
COPY web/ ./
COPY --from=rust /aoc2020/pkg/ ../aoc2020/pkg

# Run a build and export.
RUN yarn build && yarn export

# Run next start.
EXPOSE 3000
CMD ["yarn", "start"]
