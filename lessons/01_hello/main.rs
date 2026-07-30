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
    let number = 1000;
    println!("I will write rust {number}")
}
