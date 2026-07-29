# 🦀 Learn Rust — with an AI copilot

A hands-on repo for learning Rust from zero. **You** write the code; your AI copilot
(Claude Code) explains concepts, shows examples, and reviews what you write — without
just handing you the answers.

## How this works

The file [`CLAUDE.md`](./CLAUDE.md) tells the copilot exactly how to teach: explain a
concept → show a small example → give you a variation to do yourself → review your
attempt. It's tuned for a **guided walkthrough** style.

## Quick start

Rust is already installed. To begin:

```bash
cd ~/learn-rust
cargo run --bin 01_hello
```

Then open Claude Code in this folder and say: **"Let's start Lesson 1."**

## Layout

```
lessons/
  01_hello/        ← done for you as an example (has a small TODO)
  02_variables/    ← we build these together, one at a time
  03_functions/
  04_control_flow/
  05_ownership/    ← the big one
playground/        ← your scratch space, experiment freely
```

Each lesson is a separate runnable program. Run any of them with:

```bash
cargo run --bin 01_hello        # or 02_variables, 03_functions, ...
cargo run --bin playground      # your sandbox
```

## The loop you'll follow

1. Ask the copilot to start the next lesson.
2. Read the explanation + example.
3. Fill in the `// TODO` in that lesson's `main.rs`.
4. Run it, hit errors, learn from them (Rust's compiler errors are excellent).
5. Ask the copilot to review your solution.
6. Move on. Repeat.

Have fun. Making the compiler happy feels great once it clicks. 🦀
