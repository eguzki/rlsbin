# ------------------------------------------------------------------------------
# Build Stage
# ------------------------------------------------------------------------------

# Use bullseye as build image instead of Bookworm as ubi9 does not not have GLIBCXX_3.4.30
# https://access.redhat.com/solutions/6969351
FROM rust:1.79.0-bullseye AS builder

RUN apt update && apt upgrade -y \
    && apt install -y protobuf-compiler clang

WORKDIR /usr/src/rlsbin

ARG GITHUB_SHA
ARG CARGO_ARGS
ENV GITHUB_SHA=${GITHUB_SHA:-unknown}
ENV RUSTFLAGS="-C target-feature=-crt-static"

# We set the env here just to make sure that the build is invalidated if the args change
ENV CARGO_ARGS=${CARGO_ARGS}

# The following allows us to cache the Cargo dependency downloads with image layers
COPY Cargo.toml Cargo.lock ./
RUN mkdir -p src && echo 'fn main() {}' > src/main.rs
RUN cargo build --release ${CARGO_ARGS}

COPY ./build.rs ./build.rs
COPY ./src ./src
COPY ./vendor-protobufs ./vendor-protobufs

RUN cargo build --release ${CARGO_ARGS}

# ------------------------------------------------------------------------------
# Run Stage
# ------------------------------------------------------------------------------

FROM registry.access.redhat.com/ubi9/ubi-minimal:9.2

# shadow-utils is required for `useradd`
RUN PKGS="libgcc libstdc++ shadow-utils" \
    && microdnf --assumeyes install --nodocs $PKGS \
    && rpm --verify --nogroup --nouser $PKGS \
    && microdnf -y clean all
RUN useradd -u 1000 -s /bin/sh -m -d /home/rlsbin rlsbin

WORKDIR /home/rlsbin/bin/
ENV PATH="/home/rlsbin/bin:${PATH}"

COPY --from=builder /usr/src/rlsbin/target/release/rlsbin ./rlsbin

RUN chown -R rlsbin:root /home/rlsbin \
    && chmod -R 750 /home/rlsbin

USER rlsbin

CMD ["rlsbin"]
