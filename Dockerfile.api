FROM rust:latest as build

RUN mkdir /opt/auth-api
WORKDIR /opt/auth-api
COPY . .

RUN mkdir -p /app

RUN apt-get -y update && \
     apt-get -y upgrade  && \
     apt -y install cmake make libssl-dev ca-certificates libpq-dev

RUN cargo build --bin api-server --release
RUN cargo build --bin grpc-server --release
RUN cp /opt/auth-api/target/release/api-server /app
RUN cp /opt/auth-api/target/release/grpc-server /app

FROM debian:bullseye

ENV DEBIAN_FRONTEND=noninteractive
RUN mkdir -p /opt/auth-api/logs/
RUN mkdir -p /opt/auth-api/resources/
RUN mkdir -p /opt/auth-api/docs/
RUN touch /opt/auth-api/logs/api.log
RUN touch /opt/auth-api/logs/grpc.log
COPY --from=build /opt/auth-api/docs/documentation.html /opt/auth-api/docs/documentation.html
COPY --from=build /opt/auth-api/resources/resources.json /opt/auth-api/resources/resources.json
COPY --from=build /app/api-server /usr/local/bin


RUN apt-get -y update && \
     apt-get -y upgrade  && \
     apt -y install cmake make libssl-dev ca-certificates libpq-dev libc6 libc6-dev libc-bin lsb-release


ENTRYPOINT ["api-server"]
