FROM rust:latest

RUN rustup component add clippy rustfmt 
RUN rustup target add aarch64-apple-darwin
RUN rustup target add x86_64-unknown-linux-gnu
RUN rustup target add x86_64-pc-windows-msvc