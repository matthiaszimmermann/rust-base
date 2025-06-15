# Hello Cargo

## Create the Project

```bash
cd ./projects
cargo new p02_hello_cargo
cd p02_hello_cargo
```

Initial setup provides a `./Cargo.toml` file and initial source code in `./src/main.rs`. 
For more infos see [The Rust Programming Language](https://doc.rust-lang.org/book/ch01-03-hello-cargo.html#creating-a-project-with-cargo)

## Build and Run the Generated Program

```bash
cargo build
```

The build step creates a `Cargo.lock` file and additional artifacts in folder `./target`. 
The executable created by cargo build can be found under `./target/debug/p02_hello_cargo`. 

The command below simply runs that executable.
```bash
cargo run
```

Compiling and building the executable can take time for larger programs/libraries. 
Just checking if everything will compile fine can be achieved much quicker than with `cargo build`:
```bash
cargo check
```

## Build an Optimized Executable

```bash
cargo build --release
```

This command produces `./target/release/p02_hello_cargo`. 

## Cargo as Convention

Building Rust projects should always work as follows:

```bash
git clone <git-url>/<project-name>
cd <project-name>
cargo build
```bash
