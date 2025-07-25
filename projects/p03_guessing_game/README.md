# Guessing Game

From [The Rust Programming Language](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html)

## Create the Project

```bash
cd ./projects
cargo new p03_guessing_game
cd p03_guessing_game
```

## Add Rand Libraray

Add `rand = "0.9.1"` below the `[dependencies]` section in the `./Cargo.toml` file.
This results in the file content shown below.

```
[package]
name = "p03_guessing_game"
version = "0.1.0"
edition = "2024"

[dependencies]
rand = "0.9.1"
```

## Generate and View Documentation

The command below generates program specific documentation.

```bash
cargo doc
```

The documentatation is generated into folder `target/doc`.
To directly open the documentation an additional parameter would do that: 
`cargo doc --open`. However, this does not work in our devcontainer setup.

As a workaround we can "manually" serve the generated files with a Python HTTP server as shown below.

```bash
cd target/doc
python3 -m http.server 8000
```

VS Code exposes the (possibly mapped) port externally so the documentation becomes available in the browser. 

http://localhost:65167/p03_guessing_game/

Your port might differ from the example above. Check the "PORTS" tab of VS Code for the likely port.


## Run the Guessing Game

```bash
cargo run
```
