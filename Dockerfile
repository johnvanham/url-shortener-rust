FROM alpine:latest

RUN addgroup -S urlshortener && adduser -S urlshortener -G urlshortener

RUN mkdir /app

COPY --chown=urlshortener:urlshortener ./dist/url-shortener-rust /app/url-shortener
COPY --chown=urlshortener:urlshortener ./public /app/public

RUN chmod +x /app/url-shortener

USER urlshortener
WORKDIR /app

CMD [ "./url-shortener" ]