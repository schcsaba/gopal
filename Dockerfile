# Get started with a build env with Rust nightly compatible with Leptos 0.8.5
FROM rustlang/rust:nightly-bookworm AS builder

# If you’re using stable, use this instead
# FROM rust:1.88-bookworm as builder  # Leptos 0.8.5 requires Rust 1.88+

RUN apt-get update -y && \
    apt-get install -y curl

RUN curl -fsSL https://deb.nodesource.com/setup_22.x -o nodesource_setup.sh && \
    bash nodesource_setup.sh && \
    apt-get install -y nodejs

# Install cargo-binstall, which makes it easier to install other
# cargo extensions like cargo-leptos
RUN wget https://github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-x86_64-unknown-linux-musl.tgz
RUN tar -xvf cargo-binstall-x86_64-unknown-linux-musl.tgz
RUN cp cargo-binstall /usr/local/cargo/bin

# Install cargo-leptos (will get latest version compatible with Leptos 0.8.5)
RUN cargo binstall cargo-leptos -y

# Add the WASM target
RUN rustup target add wasm32-unknown-unknown

# Make an /app dir, which everything will eventually live in
RUN mkdir -p /app
WORKDIR /app

# Copy essential config files first for better layer caching and linker fix
COPY rust-toolchain.toml ./
COPY .cargo/ ./.cargo/
COPY Cargo.toml Cargo.lock ./

# Copy source code and assets
COPY src/ ./src/
COPY style/ ./style/
COPY public/ ./public/

ENV LEPTOS_TAILWIND_VERSION=v4.0.0
RUN npm install tailwindcss

# Build the app
RUN cargo leptos build --release -vv

FROM debian:bookworm-slim AS runtime
WORKDIR /app
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

# Copy the server binary to the /app directory
COPY --from=builder /app/target/release/gopal /app/

# /target/site contains our JS/WASM/CSS, etc.
COPY --from=builder /app/target/site /app/site

# Copy Cargo.toml if it’s needed at runtime
COPY --from=builder /app/Cargo.toml /app/

RUN mkdir -p /app/site/cache/image

# Set any required env variables and
ENV RUST_LOG="info"
ENV LEPTOS_SITE_ADDR="0.0.0.0:8080"
ENV LEPTOS_SITE_ROOT="site"
ENV GALLERY_PATH="/app/site/assets/img/gallery"
ENV IF_SHOW_ANNOUNCEMENT_BUTTON=false
EXPOSE 8080

# Run the server
CMD ["/app/gopal"]
