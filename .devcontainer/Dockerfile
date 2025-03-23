FROM rust:latest

RUN apt-get update && apt-get install -y \
    lld \
    clang \
    cmake \
    gnuplot \
    && rm -rf /var/lib/apt/lists/*

RUN cargo install cargo-watch

# include gnuplot
ENV PATH="/usr/bin:${PATH}"