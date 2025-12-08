FROM alpine:latest AS builder
RUN apk upgrade -U && apk add alpine-sdk bash brotli curl git nodejs npm zstd && \
    curl https://sh.rustup.rs -sSf | sh -s -- -y --profile minimal
ENV PATH="/root/.cargo/bin:${PATH}"
WORKDIR /app
COPY . .
RUN cd frontend/ && \
    npm install && npm run build && \
    ./compress-statics && \
    cd .. && \
    cargo install --locked --path . --root ./out

FROM alpine:latest
WORKDIR /app
RUN apk upgrade --no-cache && \
    addgroup -g 10001 app && adduser -u 10001 -G app -h /app -D app

USER app:app
COPY --from=builder /app/out/bin/* /app
COPY --from=builder /app/public /app/public

ENV LISTEN="[::]:8081"
EXPOSE 8081
CMD ["/app/web"]
HEALTHCHECK CMD wget -qO - http://localhost:8081/readyz || exit 1
