# Note that the following build needs binaries to be precompiled for the target
# architectures. Use the `build-all` just recipies to build for all targets.
FROM alpine as arm-builder
COPY ./target/armv7-unknown-linux-musleabihf/release/cocogitto_github_app /cocogitto_github_app

FROM alpine as arm64-builder
COPY ./target/aarch64-unknown-linux-musl/release/cocogitto_github_app /cocogitto_github_app

FROM alpine as amd64-builder
COPY ./target/x86_64-unknown-linux-musl/release/cocogitto_github_app /cocogitto_github_app

FROM ${TARGETARCH}-builder AS builder

FROM alpine
MAINTAINER Paul Delafosse "paul.delafosse@protonmail.com"

RUN addgroup -S cocogitto && adduser -S cocogitto -G cocogitto
USER cocogitto

# Install binaries
COPY --from=builder /cocogitto_github_app /usr/bin/cocogitto_github_app

EXPOSE 8080

COPY ./docker/entrypoint.sh /entrypoint.sh
COPY ./config.example.toml ./config.toml

CMD ["cocogitto_github_app"]
ENTRYPOINT ["/entrypoint.sh"]