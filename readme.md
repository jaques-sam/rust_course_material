# Rust Course Material

## The slides

Download them here in [Releases](https://github.com/jaques-sam/rust_course_material/releases).


## All code samples

Make sure you have rust installed:
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Run an example ./examples/variables_and_constants.rs:
```sh
cargo run --example variables_and_constants
```

To verify all example code:

```sh
cd rust_course_material
cargo clippy --workspace --all-targets --tests --examples -- -D warnings
```

Some test from the examples are explicitly failing to build.
Build all tests using

```sh
cargo test --examples
```


## Exercises

Install rustlings:

```sh
cargo install rustlings
```

Create the repo & enter:
```sh
rustlings init
cd rustlings/
```

Make a snapshot before solving exercises:
```sh
git add .
git commit -m "init"
```

You're set!
Start the exercises:
```sh
rustlings
```

Open each file from the instruction & solve them one by one!!

If you want to run individual tests:
```sh
rustlings hint if1  # optional hint
rustlings run if1
```
