FROM rust:1.71 AS builder
WORKDIR /opt
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo install --locked --path . --root /tmp

FROM debian:12.0-slim
WORKDIR /opt
COPY --from=builder /tmp/bin/vonage_to_matrix ./
CMD ./vonage_to_matrix
