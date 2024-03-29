FROM rust:latest as builder

WORKDIR /usr/src/rocket

# Download Litestream
ENV LITESTREAM_VERSION=v0.3.13
ADD https://github.com/benbjohnson/litestream/releases/download/$LITESTREAM_VERSION/litestream-$LITESTREAM_VERSION-linux-amd64.tar.gz /tmp/litestream.tar.gz
RUN tar -C /usr/local/bin -xzf /tmp/litestream.tar.gz

# Copy Cargo.toml. To cache dependencies.
COPY Cargo.toml ./

# Create dummy source files and build only dependencies
RUN mkdir src/ && \
    echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs
RUN cargo build --release

# Copy the real source code
COPY ./src ./src
COPY Rocket.toml ./
COPY local.db ./

# Delete dummy source files and rebuild with real sources
ENV DATABASE_URL=sqlite:./local.db
RUN touch src/main.rs && \
    cargo build --release

#FROM gcr.io/distroless/cc-debian12
FROM debian:12

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8080
ENV DATABASE_URL=sqlite:./local.db

# Copy compiled binaries from builder stage
COPY --from=builder /usr/src/rocket/target/release/rocket .
# Copy local database files
COPY --from=builder /usr/src/rocket/local.db .

RUN apt-get update && \
    apt-get install -y ca-certificates openssl && \
    rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/bin/litestream /usr/local/bin/litestream
COPY litestream.yml /etc/litestream.yml
COPY run.sh .
RUN chmod +x run.sh

EXPOSE 8080

CMD ["./run.sh"]
