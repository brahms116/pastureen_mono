FROM rust:1.70.0-slim-buster as builder
WORKDIR /usr/src/app
RUN apt-get update && apt-get install pkg-config libssl-dev -y
COPY ./rust .
RUN cargo build --release

FROM debian:buster-slim as router_service
RUN apt-get update && apt-get install libssl-dev ca-certificates -y
COPY --from=builder /usr/src/app/target/release/router_service /usr/local/bin/router_service
ENTRYPOINT ["/usr/local/bin/router_service"]

FROM debian:buster-slim as router_service_double
RUN apt-get update && apt-get install libssl-dev ca-certificates -y
COPY --from=builder /usr/src/app/target/release/router_service /usr/local/bin/router_service
ENTRYPOINT ["/usr/local/bin/router_service"]








