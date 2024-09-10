fn main() {
    // Define a closure that captures a variable from the environment.
    let x = 5;
    
    // Closures can infer types, no need to annotate parameter or return type.
    // This closure captures the value of `x` by reference and adds `y` to it.
    let add_to_x = |y| x + y;
    
    // Invoke the closure with different values for `y`.
    println!("5 + 2 = {}", add_to_x(2)); // Output: 5 + 2 = 7
    println!("5 + 3 = {}", add_to_x(3)); // Output: 5 + 3 = 8
    
    // Example of a closure with explicit type annotations.
    let multiply = |a: i32, b: i32| -> i32 { a * b };
    println!("4 * 3 = {}", multiply(4, 3)); // Output: 4 * 3 = 12

    // Demonstrate capturing ownership by moving a value into a closure.
    let s1 = String::from("hello");
    let move_closure = move || println!("String moved: {}", s1);
    // Uncommenting below would cause a compile-time error because `s1` is moved.
    // println!("{}", s1); 

    // Call the closure that has ownership of `s1`.
    move_closure(); // Output: String moved: hello

    // Closures can be stored in variables and called later.
    let square = |num: i32| num * num;
    let squared_value = square(4); // Call the closure with an argument
    println!("Square of 4: {}", squared_value); // Output: Square of 4: 16

    // Passing closures to functions
    apply_to_value(10, |x| x * 2); // Output: Result of closure: 20

    // Return a closure from a function
    let closure = returns_closure();
    println!("Closure output: {}", closure(4)); // Output: Closure output: 8
}

// Function that takes a closure as a parameter.
// The closure must take an i32 as an argument and return an i32.
fn apply_to_value<F>(value: i32, func: F)
where
    F: Fn(i32) -> i32,
{
    let result = func(value);
    println!("Result of closure: {}", result);
}

// Function that returns a closure.
// The returned closure multiplies its input by 2.
fn returns_closure() -> impl Fn(i32) -> i32 {
    |x| x * 2
}
