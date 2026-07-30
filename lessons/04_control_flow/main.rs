// Lesson 04 — Control flow (if / loop / while / for / match)
//     cargo run --bin 04_control_flow
//
// Four ways to branch and three ways to loop. The theme from Lesson 3 carries
// over: most of these are EXPRESSIONS, so they produce values you can bind.

fn main() {
    // ---- 1. if / else if / else ----

    let n = 7;
    if n < 5 {
        println!("{n} is small");
    } else if n < 10 {
        println!("{n} is medium");
    } else {
        println!("{n} is large");
    }

    // The condition MUST be a bool. Rust has no "truthiness" —
    // `if n { ... }` is an error, unlike C/Python/JS. Be explicit.

    // `if` as an expression (from Lesson 3), used on the right of a `let`:
    let label = if n % 2 == 0 { "even" } else { "odd" };
    println!("{n} is {label}");

    // ---- 2. loop — infinite until you `break` ----

    let mut countdown = 3;
    loop {
        println!("loop tick: {countdown}");
        countdown -= 1;
        if countdown == 0 {
            break; // without this, it runs forever
        }
    }

    // `break` can carry a VALUE out, making `loop` an expression.
    // This is unique to `loop` — `while` and `for` can't do it.
    let mut i = 1;
    let first_square_over_50 = loop {
        if i * i > 50 {
            break i * i; // this value becomes the loop's value
        }
        i += 1;
    };
    println!("first square over 50: {first_square_over_50}");

    // ---- 3. while — loop as long as a condition holds ----

    let mut fuel = 3;
    while fuel > 0 {
        println!("while: fuel = {fuel}");
        fuel -= 1;
    }

    // ---- 4. for — iterate over a collection or range ----

    // `0..3` is a RANGE: 0, 1, 2 — the end is EXCLUSIVE.
    for i in 0..3 {
        println!("for exclusive: {i}");
    }

    // `0..=2` includes the end. The `=` is easy to miss and easy to need.
    for i in 0..=2 {
        println!("for inclusive: {i}");
    }

    // Iterating an array. `&` borrows it — Lesson 5 explains why that matters.
    let fruits = ["apple", "banana", "cherry"];
    for fruit in &fruits {
        println!("fruit: {fruit}");
    }

    // `.enumerate()` pairs each item with its index.
    for (idx, fruit) in fruits.iter().enumerate() {
        println!("  {idx}: {fruit}");
    }

    // `for` is preferred over `while i < len` because it can't go out of bounds.
    // No off-by-one, no manual counter to get wrong.

    // ---- 5. match — the good one ----

    // Like `switch`, but the compiler FORCES you to handle every case.
    // Miss one and it won't compile. That's the feature.
    let dice = 4;
    match dice {
        1 => println!("match: rolled a one"),
        2 | 3 => println!("match: two or three"), // `|` means "or"
        4..=5 => println!("match: four or five"), // ranges work too
        _ => println!("match: six or something else"), // `_` catches the rest
    }

    // `match` is an EXPRESSION, so it produces a value.
    // Every arm must return the same type.
    let word = match dice {
        1 => "one",
        2 => "two",
        3 => "three",
        _ => "many",
    };
    println!("dice as word: {word}");

    // ---- YOUR TURN ----

    // TODO 1: Use a `for` loop over 1..=5 to print each number and its square.
    //         Example line:  3 squared is 9
    for i in 1..=5 {
        let squared = i * i;
        println!("{i} squared is {squared}");
    }

    // TODO 2: FizzBuzz, 1 to 15, using `match`.
    //         Print "Fizz" for multiples of 3, "Buzz" for multiples of 5,
    //         "FizzBuzz" for multiples of both, otherwise the number.
    //         Hint: match on the TUPLE `(n % 3, n % 5)` and pattern-match
    //         the pairs — e.g. `(0, 0) => ...`. Cleaner than nested ifs.

    for n in 1..=15 {
        match (n % 3, n % 5) {
            // Fill in FOUR arms here. Each one looks like:
            //     pattern => println!("..."),
            //
            (0, 0) => println!("FizzBuzz"),
            (0, _) => println!("Fizz"),
            (_, 0) => println!("Buzz"),
            _ => println!("{n}"),
            // Arm 1: pattern (0, 0)  -> divisible by both  -> print "FizzBuzz"
            // Arm 2: pattern (0, _)  -> divisible by 3     -> print "Fizz"
            // Arm 3: pattern (_, 0)  -> divisible by 5     -> print "Buzz"
            // Arm 4: pattern _       -> neither            -> print the number: {n}
            //
            // ORDER MATTERS: first match wins, so (0, 0) must come before (0, _).
        }
    }

    // TODO 3: Use `while` to find the first power of 2 that exceeds 1000.
    //         Print it. (Start at 1, double until it's over 1000.)
    let mut ans = 1;
    while ans <= 1000 {
        ans = ans * 2;
    }
    println!("{ans}");

    // TODO 4: Rewrite TODO 3 using `loop` with `break <value>` instead, binding
    //         the result to a variable with `let`. Compare the two — which reads
    //         better to you?
    let mut i2 = 1;
    let ans2 = loop {
        if i2 > 1000 {
            break i2;
        }
        i2 = i2 * 2;
    };
    println!("{ans2}");

    // TODO 5: Predict, THEN run. Delete the `_ =>` arm from the `match dice`
    //         block above and try to compile. Write your guess first.
    //         This error is why `match` is safer than `switch`.
}
