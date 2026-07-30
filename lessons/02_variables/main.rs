// Lesson 02 — Variables, mutability, shadowing
//
// We'll build this one together. Ask your copilot to start Lesson 2.
//     cargo run --bin 02_variables
fn main() {
    //let bindings are immutable by default
    // let x = 5 means x will never be anything but 5. Want to change it, say let mut x = 5
    //shadowing is different
    // becsause. it does not reuses the same memory slot, its a different variable, it also lets youc hange the type of your variable.

    // ---- WORKED EXAMPLE ----

    // 1. Immutable by default. This binding is frozen.
    let apples = 5;
    println!("apples: {apples}");
    // Uncomment the next line to see the compiler stop you:
    // apples = 6;

    // 2. `mut` opts in to mutation. Same variable, same memory slot, new value.
    let mut oranges = 3;
    println!("oranges before: {oranges}");
    oranges = 10; // no `let` here — we're assigning, not creating
    println!("oranges after:  {oranges}");

    // 3. Shadowing: each `let` makes a BRAND NEW variable that hides the old one.
    let spaces = "   "; // a string with 3 spaces
    let spaces = spaces.len(); // now a number! type changed str -> usize
    println!("spaces: {spaces}"); // prints 3

    // 4. Shadowing is scoped. The inner `let` only shadows inside the braces.
    let score = 100;
    {
        let score = score * 2; // new `score`, only alive in this block
        println!("inner score: {score}"); // 200
    }
    println!("outer score: {score}"); // still 100 — inner one is gone

    // 5. Constants: always immutable, need a type, SCREAMING_CASE, computed at compile time.
    const MAX_POINTS: u32 = 100_000; // underscores are just readability, ignored by the compiler
    println!("max points: {MAX_POINTS}");

    // ---- YOUR TURN ----
    // TODO 1: Make a mutable variable `count` starting at 0, then add 1 to it
    //         three times, printing it each time. (Hint: `count += 1;`)
    let mut count: i32 = 0;
    count += 1;
    println!("count is, {count}");
    count += 1;
    println!("count is, {count}");
    count += 1;
    println!("count is, {count}");
    //
    // TODO 2: Start with `let temp = "72";` (a string). Use shadowing to turn it
    //         into a number, then print double it.
    //         Hint: "72".parse::<i32>().unwrap()  <-- we'll unpack what .unwrap() means
    //                                                 in the error-handling lesson.
    let temp = "72";
    let temp = temp.parse::<i32>().unwrap();
    let temp = temp * 2;
    println!("double of, {temp}");
    //.
    // TODO 3: Predict, THEN run. Write your guess in a comment first:
    //             let y = 10;
    //             let y = y + 5;
    //             let y = y * 2;
    //         What does `y` print? Guess, then check.
    // it should be y = 10 then y = 15 then y = 30
    let y = 10;
    let y = y + 5;
    let y = y * 2;
    println!("this is {y}");
}
