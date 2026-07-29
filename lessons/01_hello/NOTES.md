# Lesson 01 — Hello, Rust

## Concept
- Every Rust program starts at `fn main()`.
- `println!` is a **macro** (note the `!`). It prints text + a newline.
- `{}` inside a string is a placeholder filled by arguments; `{name}` inlines a
  variable directly.

## Run it
```
cargo run --bin 01_hello
```

## What cargo just did
- `cargo` is Rust's build tool + package manager (like npm/pip, but built in).
- `cargo run` compiles your code with `rustc` and then runs the binary.
- Compiled output lands in `target/` (git-ignored — don't commit it).

## Your turn
Complete the TODO in `main.rs`, then run it again.

## Notes to self
_(write anything you learned or got stuck on here)_
