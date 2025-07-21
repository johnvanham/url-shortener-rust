FROM rust:1.88.0 as builder

WORKDIR /app
COPY Cargo.toml Cargo.lock Rocket.toml ./
COPY src/ ./src/
COPY css/ ./css/
COPY public/ ./public/

RUN cargo build --release

FROM registry.fedoraproject.org/fedora-minimal:latest

RUN microdnf install -y sqlite && microdnf clean all
RUN groupadd -r urlshortener && useradd -r -g urlshortener urlshortener
RUN mkdir -p /app/data && chown urlshortener:urlshortener /app/data

COPY --from=builder --chown=urlshortener:urlshortener /app/target/release/url-shortener-rust /app/url-shortener-rust
COPY --chown=urlshortener:urlshortener ./public /app/public
COPY --chown=urlshortener:urlshortener ./Rocket.toml /app/Rocket.toml

RUN chmod +x /app/url-shortener-rust

USER urlshortener
WORKDIR /app

ENV DATABASE_URL=/app/data/urls.db
ENV RUST_LOG=info

EXPOSE 7575

CMD [ "./url-shortener-rust" ]