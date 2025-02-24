FROM docker.io/rust:1.85 as builder

WORKDIR /usr/src/app
COPY . .

RUN cargo install --path .

FROM debian:bookworm-slim
RUN apt-get update -y && apt-get upgrade -y && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/local/cargo/bin/promspeed /usr/local/bin/promspeed
CMD [ "promspeed" ]
