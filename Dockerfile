FROM rust:1.71 AS builder
WORKDIR /opt
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY vonage ./vonage
COPY hookshot ./hookshot
RUN cargo install --locked --path . --root /tmp

FROM debian:bullseye-slim
RUN apt-get update -y \
  && apt-get install -y --no-install-recommends openssl ca-certificates \
  && apt-get autoremove -y \
  && apt-get clean -y \
  && rm -rf /var/lib/apt/lists/*
WORKDIR /opt
COPY --from=builder /tmp/bin/vonage_to_matrix ./
CMD ./vonage_to_matrix
