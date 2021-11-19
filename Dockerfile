FROM rust:slim-buster as builder

WORKDIR /build
COPY Cargo.toml ./
COPY src ./src
RUN ls src
RUN cargo fetch
RUN cargo build

FROM rust:slim-buster

WORKDIR /app
COPY --from=builder /build/target/debug/web-responder ./

ENTRYPOINT ["/app/web-responder"]
