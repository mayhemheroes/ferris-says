FROM ghcr.io/evanrichter/cargo-fuzz as builder

ADD . /ferris-says
WORKDIR /ferris-says
RUN cargo +nightly fuzz build && \
    cd fsays && cargo build --release

FROM debian:bookworm
COPY --from=builder \
    /ferris-says/fuzz/target/x86_64-unknown-linux-gnu/release/ferris-fuzz \
    /ferris-says/fuzz/target/x86_64-unknown-linux-gnu/release/ferris-fuzz-files \
    /

COPY /ferris-says/target/x86_64-unknown-linux-gnu/release/fsays /bin/
