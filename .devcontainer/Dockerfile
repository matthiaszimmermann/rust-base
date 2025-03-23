FROM rust:latest

RUN apt-get update && apt-get install -y \
    git \
    gnuplot \
    lld \
    clang \
    cmake \
    && rm -rf /var/lib/apt/lists/*

# add optional rust compnents 
RUN rustup component add clippy rustfmt
RUN cargo install cargo-watch

# include gnuplot
ENV PATH="/usr/bin:${PATH}"

WORKDIR /workspace