use std::fmt::Debug; // Import the Debug trait, which allows types to be formatted using `{:?}` for debugging.

// Define a trait `Area` for calculating the area of shapes.
// Any type implementing this trait must define the `area` method.
trait Area {
    fn area(&self) -> f64;  // The method `area` must return the area as an `f64`.
}

// A generic struct `Rectangle` that can take any type `T` for its width and length.
#[derive(Debug)] // Derive the `Debug` trait to allow printing the struct using `{:?}`.
struct Rectangle<T> {
    width: T,    // Width of the rectangle, of generic type `T`.
    length: T,   // Length of the rectangle, also of type `T`.
}

// Implement the `Area` trait for `Rectangle`, where `T` is a generic type.
// The `T` must implement both the `Into<f64>` and `Copy` traits.
impl<T: Into<f64> + Copy> Area for Rectangle<T> {
    fn area(&self) -> f64 {
        // Convert the width and length from `T` into `f64` and compute the area.
        self.width.into() * self.length.into()
    }
}

// A generic struct `Circle` that can take any type `T` for its radius.
#[derive(Debug)] // Derive the `Debug` trait for the `Circle` struct as well.
struct Circle<T> {
    radius: T,   // Radius of the circle, of generic type `T`.
}

// Implement the `Area` trait for `Circle`, where `T` is a generic type.
// The `T` must implement both `Into<f64>` and `Copy` traits.
impl<T: Into<f64> + Copy> Area for Circle<T> {
    fn area(&self) -> f64 {
        // Calculate the area of the circle using the formula π * r^2.
        std::f64::consts::PI * self.radius.into() * self.radius.into()
    }
}

// A generic function `print_area` that accepts any type `T` that implements both `Area` and `Debug` traits.
// This function prints the shape and its calculated area.
fn print_area<T: Area + Debug>(shape: &T) {
    println!("Area of the provided {:?} is {:?}", shape, shape.area());
}

fn main() {
    // Create a rectangle with integer dimensions (i32).
    let rect = Rectangle {
        width: 10,  // width of type i32
        length: 5,  // length of type i32
    };

    // Create a rectangle with floating point dimensions (f64).
    let rect_f64 = Rectangle {
        width: 4.5, // width of type f64
        length: 3.2, // length of type f64
    };

    // Create a circle with an integer radius (i32).
    let circle = Circle {
        radius: 7,   // radius of type i32
    };

    // Create a circle with a floating point radius (f64).
    let circle_f64 = Circle {
        radius: 3.7, // radius of type f64
    };

    // Call the `print_area` function with each shape.
    // This will print the debug output of the shape and its computed area.
    print_area(&rect);       // Rectangle with integer dimensions
    print_area(&rect_f64);   // Rectangle with floating point dimensions
    print_area(&circle);     // Circle with integer radius
    print_area(&circle_f64); // Circle with floating point radius
}
