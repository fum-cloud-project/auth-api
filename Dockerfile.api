FROM rust:latest as build

RUN mkdir /opt/auth-api
WORKDIR /opt/auth-api
COPY . .

RUN mkdir -p /app

RUN cargo build --bin api-server --release
RUN cargo build --bin grpc-server --release
RUN cp /target/release/api-server /app
RUN cp /target/release/grpc-server /app

FROM debian:buster-slim

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get -y update && \
     apt-get -y upgrade  && \
     apt -y install cmake make libssl-dev ca-certificates libpq-dev

COPY --from=build /app/api-server /usr/local/bin

ENTRYPOINT ["api-server"]
