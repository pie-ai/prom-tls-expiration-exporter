FROM rust:1.44-buster AS build

# base utils
# https://github.com/killercup/cargo-edit
RUN cargo install cargo-edit

ARG PROJECT_NAME="prom-tls-expiration-exporter"
ARG BASEDIR=/var/development/src
ARG WORKDIR=${BASEDIR}/${PROJECT_NAME}

RUN mkdir -p ${BASEDIR} ; cd ${BASEDIR} && USER=root cargo new --bin ${PROJECT_NAME}
WORKDIR ${WORKDIR}

RUN cargo add csv@1.1
RUN cargo add ssl-expiration@0.1.2
RUN cargo add --features=derive serde@1.0
RUN cargo add prometheus_exporter_base@0.30.3
RUN cargo add futures@0.1.25
RUN cargo add --features=full tokio@0.2
RUN cargo add log@0.4.8
RUN cargo add env_logger@0.7.1
RUN cargo add pico-args@0.3.4
RUN cat Cargo.toml
RUN echo "fn main() {}" > main.rs
RUN cargo build
RUN find

COPY src ./src

RUN cat Cargo.toml

RUN cargo build --release
RUN ls -la ${WORKDIR}/target/release

FROM rust:1.44-buster
ARG PROJECT_NAME="prom-tls-expiration-exporter"
ARG BASEDIR=/var/development/src
ARG WORKDIR=${BASEDIR}/${PROJECT_NAME}
COPY --from=build ${WORKDIR}/target/release/${PROJECT_NAME} rust-binary
USER 1000
CMD ["./rust-binary"]