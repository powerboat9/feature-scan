FROM alpine

RUN apk update
RUN apk upgrade
RUN apk add bash curl libgcc build-base

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | \
    sh -s -- -y --default-toolchain none

RUN . ~/.cargo/env; \
    rustup set profile minimal; \
    rustup toolchain install -c rustc,rust-std,cargo,rust-src,rustc-dev RUST_VERSION

WORKDIR /work

RUN ln -s '/root/.rustup/toolchains/RUST_VERSION-x86_64-unknown-linux-musl/lib/rustlib/rustc-src/rust/compiler' /work/compiler

RUN cd /work/compiler/rustc_feature && ~/.cargo/bin/cargo update --dry-run

COPY Cargo.toml Cargo.lock /work/feature_scan/

RUN mkdir /work/feature_scan/src && echo "fn main() {}" > /work/feature_scan/src/main.rs

RUN cd /work/feature_scan && RUSTC_BOOTSTRAP=1 ~/.cargo/bin/cargo build

COPY src/main.rs /work/feature_scan/src/

RUN cd /work/feature_scan && RUSTC_BOOTSTRAP=1 ~/.cargo/bin/cargo build

ENTRYPOINT ["/bin/sh", "-c", "RUST_BACKTRACE=1 /work/feature_scan/target/debug/feature_scan"]
