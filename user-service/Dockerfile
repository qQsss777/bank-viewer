FROM rust:1.86 as builder

WORKDIR /usr/src/app
COPY . .

RUN apt-get update && apt-get install -y pkg-config libpq-dev
RUN cargo build --release

FROM debian:bookworm
RUN apt-get update && apt-get install -y libpq5 ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/app/target/release/user-service /usr/local/bin/user-service

ENV PG_HOST=db
ENV PG_USER=postgres
ENV PG_PASSWORD=postgres
ENV PG_DATABASE=users
ENV PG_SCHEMA=users
ENV PG_TABLE=users
ENV SERVICE_HOST=0.0.0.0
ENV SERVICE_PORT=3003
CMD ["user-service"]
