FROM rust:1.57.0-slim-buster

WORKDIR /api

RUN apt-get -y update \
&& apt-get install -y \
     libpq-dev \
     default-libmysqlclient-dev \
&& cargo install diesel_cli --no-default-features --features mysql \
&& cargo install cargo-watch

COPY ./src /api/src
COPY ./Cargo.toml /api
COPY ./.env /api

ENV CARGO_BUILD_TARGET_DIR=/api/target

EXPOSE 80