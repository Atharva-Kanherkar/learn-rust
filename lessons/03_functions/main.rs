// Lesson 03 — Functions & expressions
//     cargo run --bin 03_functions
//
// The big idea: in Rust, almost everything is an EXPRESSION (it produces a value).
// The semicolon is what THROWS that value away.

// ---- WORKED EXAMPLE ----

// A function with no return value. Parameter types are always required —
// Rust never guesses what you meant at a function boundary.
fn greet(name: &str) {
    println!("Hello, {name}!");
}

// `-> i32` declares the return type. Note: NO semicolon on `a + b`.
// That last expression IS the return value. This is the normal Rust style.
fn add(a: i32, b: i32) -> i32 {
    a + b // <-- no semicolon. Add one and this stops compiling.
}

// The same thing with an explicit `return`. Legal, but reserved for
// bailing out EARLY. Idiomatic Rust uses the tail expression above.
fn add_verbose(a: i32, b: i32) -> i32 {
    return a + b;
}

// `return` earning its keep: leaving the function before the end.
fn absolute(n: i32) -> i32 {
    if n < 0 {
        return -n; // bail out early
    }
    n // otherwise fall through to the tail expression
}

// `if` is an EXPRESSION, so it can produce a value directly.
// Both branches must have the SAME type — the compiler needs one type for the result.
fn describe(n: i32) -> &'static str {
    if n % 2 == 0 {
        "even" // no semicolon — this is the block's value
    } else {
        "odd"
    }
}

// A BLOCK is also an expression. `{ ... }` evaluates to its last expression.
fn block_demo() -> i32 {
    let x = {
        let inner = 4;
        inner * inner // the block evaluates to 16
    }; // this semicolon ends the `let` statement, not the block
    x + 1
}

fn main() {
    greet("learner");

    // Calling a function that returns a value — we can use it like any other value.
    let sum = add(3, 4);
    println!("add(3, 4)         = {sum}");
    println!("add_verbose(3, 4) = {}", add_verbose(3, 4));

    println!("absolute(-7)      = {}", absolute(-7));
    println!("absolute(7)       = {}", absolute(7));

    // Using an `if` expression on the right-hand side of a `let`.
    let n = 10;
    println!("{n} is {}", describe(n));

    println!("block_demo()      = {}", block_demo());

    // Statement vs expression, the punchline:
    //   `let y = 6;`  is a STATEMENT — it does something, produces no value.
    //   `6`           is an EXPRESSION — it IS a value.
    // That's why `let x = (let y = 6);` is an error: nothing to bind.

    // ---- YOUR TURN ----

    // TODO 1: Write a function `double(n: i32) -> i32` that returns n * 2.
    //         Use the tail-expression style (no `return`, no semicolon).
    //         Then call it here and print the result.
    fn double(n: i32) -> i32 {
        n * 2
    }
    let doub = double(12);
    println!("n value is, {doub}");
    // TODO 2: Write `is_even(n: i32) -> bool` that returns true for even numbers.
    //         Hint: `n % 2 == 0` is already a bool — just make it the tail expression.
    //         No `if` needed at all. Call it with 4 and 7 and print both.
    fn is_even(n: i32) -> bool {
        n % 2 == 0
    }
    let is4even: bool = is_even(4);
    println!("is 4even {is4even}");
    // TODO 3: Write `fizz(n: i32) -> &'static str` returning "fizz" if n is
    //         divisible by 3, otherwise "no". Use an `if` EXPRESSION as the
    //         tail (like `describe` above) — not a `return`.

    fn fizz(n: i32) -> &'static str {
        if n % 3 == 0 {
            return "yes";
        }
        "no"
    }

    // TODO 4: Predict, THEN run. What happens if you add a semicolon to `a + b`
    //         inside `add`? Write your guess as a comment, then try it, read the
    //         error, and put it back. This is the most important error in this lesson.
    // maybe it willl just throw an error bcs it will say it was expecting some value but you threw it away so it got ()
}
