FROM rust:1.88

RUN apt-get update && apt-get install -y \
    git \
    gnuplot \
    lld \
    clang \
    cmake \
    lldb \
    pkg-config \
    sudo \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user with sudo access
ARG USERNAME=devuser
ARG USER_UID=1000
ARG USER_GID=$USER_UID

RUN groupadd --gid $USER_GID $USERNAME && \
    useradd --uid $USER_UID --gid $USER_GID -m $USERNAME && \
    echo "$USERNAME ALL=(ALL) NOPASSWD:ALL" >> /etc/sudoers.d/$USERNAME && \
    chmod 0440 /etc/sudoers.d/$USERNAME

# Set user-owned Cargo home
ENV CARGO_HOME=/home/$USERNAME/.cargo
RUN mkdir -p $CARGO_HOME && chown -R $USERNAME:$USERNAME $CARGO_HOME

# Install Rust components and tools as the user
USER $USERNAME
RUN rustup component add clippy rustfmt && \
    cargo install cargo-watch

# Set working directory
WORKDIR /workspace