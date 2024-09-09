fn main() {
    // Define a string literal with a static lifetime. This means the string will be valid
    // for the entire duration of the program's execution.
    let Longest: &'static str = "longest";

    // Create two heap-allocated strings using the String type.
    let s1 = String::from("hello world");
    let s2 = String::from("hello rust");

    // Call the `longest` function, passing references to `s1` and `s2`.
    // Because we pass references, Rust borrows the values and ensures
    // ownership remains with `s1` and `s2`. The result is also a reference.
    let result = longest(&s1, &s2);

    // Print the result, which will be the longest string between `s1` and `s2`.
    println!("{:?}", result);

    // Example of struct usage with a borrowed reference.
    let excerpt = ImportantExcerpt { part: &s1 };

    // Print the excerpt's part, which borrows from `s1`.
    println!("{:?}", excerpt.part);

    // Example of function without explicit lifetime annotations, relying on Rust's
    // lifetime elision rules. This function works due to Rust's automatic inference.
    println!("{:?}", first_word(&s1));
}

// Define a function `longest` that takes two string references (`x` and `y`) with
// the same lifetime `'a`. The return type is also a reference that shares the same
// lifetime as the input parameters.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x // Returns `x`, which must live as long as the lifetime `'a`.
    } else {
        y // Returns `y`, which must also live as long as the lifetime `'a`.
    }
}

// Define a struct `ImportantExcerpt` that holds a reference to a part of a string.
// The struct's reference has a lifetime parameter `'a`, meaning the reference inside
// the struct must not outlive the data it points to.
struct ImportantExcerpt<'a> {
    part: &'a str, // `part` must have the same lifetime `'a` as the string it references.
}

// Function without explicit lifetime annotations. Rust applies lifetime elision rules
// here, automatically inferring the lifetimes. There are three elision rules:
// 1. Each parameter with a reference gets its own lifetime.
// 2. If there's only one input reference, its lifetime is assigned to the output.
// 3. If multiple input references exist, but one is `&self` or `&mut self`, that lifetime is used.
fn first_word(s: &str) -> &str {
    // This function uses lifetime elision: Rust implicitly assigns lifetimes to
    // both the input (`s`) and output (&str). The lifetime of the returned reference
    // will match the lifetime of `s`.
    &s[0..1] // Returns a slice of the first character (this is just an example).
}

// Demonstrate lifetime variance: The function accepts any reference that lives for at least `'a`.
// This allows longer-lived references to be passed in.
// If `longest_static` took `'static` references, it could only accept string literals.
fn longest_static<'a>(x: &'a str, y: &'static str) -> &'a str {
    // Here, `y` is a reference with the `'static` lifetime (it lives forever),
    // but we can still return it as `'a`, which may be a shorter lifetime.
    if x.len() > y.len() {
        x
    } else {
        y // This works because `'static` is longer than any possible `'a`.
    }
}
