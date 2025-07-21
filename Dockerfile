FROM rust:1.88.0 as builder

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src/ ./src/
COPY css/ ./css/
COPY public/ ./public/

RUN cargo build --release

FROM alpine:latest

RUN addgroup -S urlshortener && adduser -S urlshortener -G urlshortener
RUN mkdir -p /app/data

COPY --from=builder --chown=urlshortener:urlshortener /app/target/release/url-shortener-rust /app/url-shortener
COPY --chown=urlshortener:urlshortener ./public /app/public

RUN chmod +x /app/url-shortener

USER urlshortener
WORKDIR /app

ENV DATABASE_URL=/app/data/urls.db
ENV RUST_LOG=info

EXPOSE 8000

CMD [ "./url-shortener" ]