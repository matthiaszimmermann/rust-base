# Rust Development Base Repository

A Rust development environment with VS Code Dev Containers, featuring Serde for JSON serialization/deserialization.

## Setup

1. Install [Docker](https://www.docker.com/get-started) and [VS Code](https://code.visualstudio.com/)
2. Install the [Remote - Containers](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers) extension in VS Code
3. Clone this repository: 
```bash
git clone https://github.com/matthiaszimmermann/rust-base.git
```
4. Open the project in VS Code:
```bash
code rust-base
```
5. When prompted, click "Reopen in Container"

## Usage

Once the dev container is built and running, you can use the integrated terminal in VS Code to run Rust commands.

### Running Tests

To run all tests:
```bash
cargo test
```

### Running Examples
```bash
cargo run --example serde_example
```

### Running Benchmarks
```bash
cargo bench
```

### Build and Run the Executable

1. Build the main executable
```bash
cargo build --release
```
2. Add the cargo bin path `/usr/local/cargo/bin/` to the PATH variable
```bash
cargo install --path .
```
3. Run the application (see section `bin` in `Cargo.toml`)
```bash
user 42 "Bob Builder" bob@example.com
user '{"id":42,"name":"Bob Builder","email":"bob@example.com"}'
```