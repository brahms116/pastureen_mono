FROM public.ecr.aws/lambda/provided:al2.2023.07.10.09 as rust_lambda_builder

RUN yum install -y jq openssl-devel gcc zip
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \
    | CARGO_HOME=/cargo RUSTUP_HOME=/rustup sh -s -- -y --profile minimal --default-toolchain stable \
    && source /cargo/env \
    && rustup default stable

WORKDIR /app
COPY ./rust .

RUN source /cargo/env \
    && cargo build --release

WORKDIR /

RUN mkdir zip
RUN mkdir out
RUN cp /app/target/release/router_service /zip/bootstrap
RUN zip -j /out/router_service.zip /zip/bootstrap

RUN cp /app/target/release/router_service /zip/bootstrap
RUN zip -j  /out/router_service_double.zip /zip/bootstrap

FROM alpine:3
COPY --from=rust_lambda_builder /out /build/rust_lambda/

