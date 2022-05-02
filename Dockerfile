FROM rust:latest
RUN mkdir /opt/auth-api
WORKDIR /opt/auth-api
COPY . .

RUN mkdir -p /app

RUN apt-get -y update && \
     apt-get -y upgrade  && \
     apt -y install cmake make libssl-dev ca-certificates libpq-dev
RUN cargo build --bin api-server --release


RUN cp /opt/auth-api/target/release/api-server /usr/local/bin

ENTRYPOINT ["api-server"]