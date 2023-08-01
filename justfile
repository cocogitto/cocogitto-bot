## Build
# needed between cross build, otherwise some link to GLIC are broken
clean-targets:
    rm -rd target/release
    rm target/.rustc_info.json

build-x86:
    cross build --target x86_64-unknown-linux-musl --release
    just clean-targets

build-arm-v7:
    cross build --target armv7-unknown-linux-musleabihf --release
    just clean-targets

build-arm-64:
    cross build --target aarch64-unknown-linux-musl --release
    just clean-targets

build-all: build-x86 build-arm-v7 build-arm-64

docker-build: build-all
    docker buildx build --no-cache --push --platform linux/amd64,linux/arm/v7,linux/arm64/v8  . -t oknozor/cocogitto-bot:latest