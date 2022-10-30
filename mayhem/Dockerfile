FROM ghcr.io/evanrichter/cargo-fuzz as builder

ADD . /tablam
WORKDIR /tablam/fuzz
RUN cargo +nightly fuzz build 

FROM debian:bookworm
COPY --from=builder /tablam/fuzz/target/x86_64-unknown-linux-gnu/release/tablam-parse-fuzz /