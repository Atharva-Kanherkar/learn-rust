// Lesson 01 — Hello, Rust!
//
// This is a complete, worked example. Just run it:
//
//     cargo run --bin 01_hello
//
// `fn main()` is the entry point of every Rust program — where execution starts.
fn main() {
    // `println!` is a MACRO (the `!` tells you so), not a function.
    // Macros generate code at compile time. This one prints a line to the screen.
    println!("Hello, Rust!");

    // `{}` is a placeholder that gets filled in by the argument after the string.
    let name = "learner";
    println!("Welcome, {name}!"); // Rust can inline variables right inside the braces.

    // ---- YOUR TURN ----
    // TODO: Add a line that prints a number using a `{}` placeholder.
    // Example goal output:  I will write 1000 lines of Rust: 1000
    // Try it yourself, then run the program again and tell your copilot how it went.
}
