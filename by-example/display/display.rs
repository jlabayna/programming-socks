/*
 * Topics:
 * - derive
 * - std::fmt
 * - macros
 * - struct
 * - trait
 * - use
 */

/*
 * Customize output by implementing fmt::Display.
 * Uses `{}` print marker.
 */

// Import `fmt` module
use std::fmt;

// Define struct for example
struct Structure(i32);

// Implement `fmt::Display` trait to allow use of `{}` marker:
impl fmt::Display for Structure {
    // Must use `fmt` with this signature:
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write first element into output stream `f`
        // Returns `fmt::Result` for success/failure
        write!(f, "{}", self.0)
    }
}

/*
 * Q: How to implement printing for ambiguous, generic types
 * (which may differ based on circumstance, i.e int vs string vector)?
 * A: Must use `fmt::Debug`
 *
 * Q: How to implement printing for ambiguous, non-generic types?
 * A: `fmt::Display`
 */

// MinMax holds two numbers
#[derive(Debug)]
struct MinMax(i64, i64);


impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // `self.<num>` refers to each data point based on its position
        write!(f, "({},{})", self.0, self.1)
    }
}

// Point2D has named fields
#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64,
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} + {}i", self.real, self.imag)
    }
}

// Example program
fn main() {
    let minmax = MinMax(0, 14);

    println!("Compare structures:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range =   MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!("The big range is {big} and the small is {small}",
             small = small_range,
             big = big_range);

    let point = Point2D { x: 3.3, y: 7.2 };

    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);

    let cnum = Complex { real: 3.3, imag: 7.2 };

    println!("Another comparison:");
    println!("Display: {}", cnum);
    println!("Debug: {:?}", cnum);

    // Must implement traits, like binary (`{:b}`) to use them.
    // See std::fmt for more.
}
