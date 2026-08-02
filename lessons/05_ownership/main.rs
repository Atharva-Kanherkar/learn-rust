// Lesson 05 — Ownership (part 1 of 2: ownership & moves)
//     cargo run --bin 05_ownership
//
// THE ONE RULE:
//   Every value has exactly ONE owner. When the owner goes out of scope,
//   the value is freed. Ownership can be MOVED, but never shared by accident.
//
// Why: memory has to be freed exactly once. C makes you do it by hand (forget =
// leak, do it twice = crash). Java/Python/Go use a garbage collector (a runtime
// that pauses your program to check). Rust does neither — the compiler proves at
// COMPILE TIME who frees what. Zero runtime cost, no GC, no leaks.

fn main() {
    // ---- 1. Scope: the owner dies at the closing brace ----

    {
        let s = String::from("hello"); // s owns this string
        println!("1. inside the block: {s}");
    } // <-- s goes out of scope HERE. Rust frees the string automatically.
      // Using `s` after this line is a compile error — it no longer exists.

    // ---- 2. Two kinds of types, and this is the crux ----

    // (a) Simple, fixed-size values live on the STACK and are COPIED.
    let a = 5;
    let b = a; // b is a copy. Both work fine.
    println!("2a. copy: a = {a}, b = {b}");

    // Integers, floats, bools, chars, and tuples of those are `Copy`:
    // copying them is trivially cheap, so Rust just does it.

    // (b) Growable, heap-allocated values are MOVED, not copied.
    let s1 = String::from("hello");
    let s2 = s1; // ownership MOVES from s1 to s2. s1 is now invalid!
    println!("2b. move: s2 = {s2}");

    // Why not copy? A String's text lives on the heap. Copying it would mean
    // either (1) duplicating the text (slow, and you didn't ask for it), or
    // (2) two owners pointing at the same text — and then BOTH would try to
    // free it at scope end. That's a double-free bug. Rust picks a third
    // option: move ownership, and invalidate the old name.

    // Want a real duplicate? Ask for it explicitly with .clone():
    let s3 = String::from("world");
    let s4 = s3.clone(); // now there are two independent strings
    println!("2b. clone: s3 = {s3}, s4 = {s4}"); // both still valid

    // ---- 3. Passing to a function moves ownership too ----

    let text = String::from("taken");
    takes_ownership(text); // `text` is moved INTO the function
                           // println!("{text}");  // <-- would not compile: text was moved away

    let num = 42;
    makes_copy(num); // i32 is Copy, so this is a copy
    println!("3. num is still usable: {num}"); // still fine!

    // ---- 4. A function can give ownership back ----

    let given = gives_ownership(); // the function's String moves out to us
    println!("4. got ownership of: {given}");

    let sent = String::from("round trip");
    let returned = takes_and_gives_back(sent); // moved in, then moved back out
    println!("4. round trip: {returned}");

    // Notice how clumsy that is — you have to keep handing the value back just
    // to keep using it. That's exactly the problem BORROWING solves, and that's
    // part 2 of this lesson.

    // ---- YOUR TURN ----

    // TODO 1: Predict, THEN run. Uncomment line 36 (`println!("{s1}")`).
    //         Write your guess first: what does the compiler say, and why?
    //         Read the error carefully — it uses the words "moved" and
    //         "borrow of moved value". Then re-comment it.
    // the compiler will say, s1 is not defined. is my honest opinion because it is just not defined, now owneship has moved away from s1 to s2.
    //to be honest now the compiler just doesnt know what s1 is now. vlaue has moved, and it does nmot implement the copy trait. because it does not,
    //it has to be either moved. not BORROWED

    // TODO 2: Fix this broken code by adding ONE method call.
    //         Uncomment these three lines and make them work:
    //
    let original = String::from("keep me");
    let copy = original.clone();

    println!("original = {original}, copy = {copy}");

    //
    //         Hint: you want two independent strings, not a move.

    // TODO 3: This function call breaks the line after it. Uncomment and fix
    //         it by making `takes_ownership` GIVE THE STRING BACK (use
    //         `takes_and_gives_back` instead, and bind the result):
    //
    let mine = String::from("mine");
    // `takes_and_gives_back` returns the String, and we SHADOW `mine` with it
    // (Lesson 2) — so the name `mine` now owns the value that came back out.
    let mine = takes_and_gives_back(mine);
    println!("still mine: {mine}");

    // TODO 4: Predict, THEN run. Does this compile? Guess before trying.
    //         Write down WHY for each of the two lines.
    //
    // let x = 5;
    // let y = x;
    // println!("{x} and {y}");
    // yes it will compile because now the owennship has been copied, not moved
    // let p = String::from("hi");
    // let q = p;
    // println!("{p} and {q}");
    // nah man, this will not work for a simple reason that p does not exist anymore, ti is invalid.
}

// Takes ownership: the String moves in here and dies at this function's end.
fn takes_ownership(some_string: String) {
    println!("   takes_ownership got: {some_string}");
} // <-- some_string is freed here

// i32 is Copy, so the caller keeps their original.
fn makes_copy(some_number: i32) {
    println!("   makes_copy got: {some_number}");
}

// Creates a String and moves ownership OUT to whoever called it.
fn gives_ownership() -> String {
    String::from("a gift") // tail expression (Lesson 3!) — moves out
}

// Takes ownership, then returns it so the caller can keep using it.
fn takes_and_gives_back(some_string: String) -> String {
    some_string // hand it right back
}
