FROM rust:1-alpine as build

RUN apk add --no-cache musl-dev
RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /project

COPY . .

ENV CARGO_HOME=/usr/local/cargo

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/project/target \
    cargo build --release --target x86_64-unknown-linux-musl
RUN --mount=type=cache,target=/project/target \
    cp /project/target/x86_64-unknown-linux-musl/release/ip-address-updater /ip-address-updater

FROM scratch

COPY --from=build /ip-address-updater /ip-address-updater

CMD ["/ip-address-updater"]
