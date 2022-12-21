FROM ghcr.io/evanrichter/cargo-fuzz as builder

ADD . /ferris-says
WORKDIR /ferris-says/fuzz
RUN cargo +nightly fuzz build 

FROM debian:bookworm
COPY --from=builder /ferris-says/fuzz/target/x86_64-unknown-linux-gnu/release/ferris-fuzz /
