FROM rust:1.82 as builder

WORKDIR /app

COPY Cargo.toml .
COPY src ./src
COPY migrations ./migrations

RUN cargo build --release

FROM debian:bookworm-slim

RUN useradd -m appuser
WORKDIR /app

COPY --from=builder /app/target/release/security-ux-backend /usr/local/bin/security-ux-backend

ENV RUST_LOG=info

USER appuser

EXPOSE 8080

CMD ["security-ux-backend"]


