FROM rust:latest as builder
RUN apt-get update

WORKDIR /app

COPY . .

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    cargo build --release && \
     cp /app/target/release/talk_hub /talk_hub

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y libssl-dev libpq-dev

RUN PATH=$PATH:/usr/local/pgsql/bin

COPY --from=builder /talk_hub /bin/talk_hub

CMD ["talk_hub"]