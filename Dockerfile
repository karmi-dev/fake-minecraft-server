# build stage
FROM rust:slim AS builder
WORKDIR /code

COPY Cargo.toml Cargo.lock ./

# caches cargo dependencies
RUN cargo fetch

# copy server source code
COPY src src

# build release binary
RUN cargo build --release

# run stage
FROM gcr.io/distroless/cc:latest

# copy server binary from build stage
COPY --from=builder /code/target/release/fake-server /usr/local/bin/fake-server

# start server
ENTRYPOINT [ "/usr/local/bin/fake-server" ]
