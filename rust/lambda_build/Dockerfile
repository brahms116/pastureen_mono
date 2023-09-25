FROM public.ecr.aws/lambda/provided:al2.2023.07.10.09

RUN yum install -y jq openssl-devel gcc zip 

# Install protobuf compiler
RUN PB_REL="https://github.com/protocolbuffers/protobuf/releases" && \
    curl -LO $PB_REL/download/v3.15.8/protoc-3.15.8-linux-x86_64.zip  && \
    unzip protoc.zip -d /usr/local && \
    rm -f protoc.zip

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \
    | CARGO_HOME=/cargo RUSTUP_HOME=/rustup sh -s -- -y --profile minimal --default-toolchain stable \
    && source /cargo/env \
    && rustup default stable

WORKDIR /build

COPY build.sh .

WORKDIR /app

ENTRYPOINT ["/bin/bash", "/build/build.sh"]
