FROM rust:1.85 AS builder

WORKDIR /app

COPY ./Cargo.toml ./Cargo.lock ./

COPY ./src ./src

RUN cargo build --release

COPY ./uploads ./uploads

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/web_service_example .
COPY --from=builder /app/uploads ./uploads

EXPOSE 3000

CMD [ "./web_service_example" ]