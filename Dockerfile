FROM rust:1.72 as builder

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    build-essential \
    wget \
    libclang-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app

COPY . .

RUN cargo install --path ./bin/katana --locked --force

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    libc6 \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/local/cargo/bin/katana /usr/local/bin/katana

ENTRYPOINT ["/usr/local/bin/katana", "--http.addr", "0.0.0.0"]
