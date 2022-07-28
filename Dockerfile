FROM rust:1.62-buster

WORKDIR /api

RUN apt-get -y update \
&& apt-get install -y \
     libpq-dev \
&& cargo install diesel_cli --no-default-features --features postgres \
&& cargo install cargo-watch

COPY ./src /api/src
COPY ./Cargo.toml /api
COPY ./.env /api

ENV CARGO_BUILD_TARGET_DIR=/api/target

EXPOSE 80