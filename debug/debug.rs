/* Topics:
 * - attributes
 * - derive
 * - std::fmt
 * - struct
 */

/*
 * Types using `std::fmt` formatting traits must be printable.
 * Automatic implementations only provided for types in `std`.
 *
 * `fmt::Debug trait`:
 * - All types can `derive` fmt::Debug implementation
 * - fmt::Display must be manually implemented
 */

// This structure cannot be printed either with `fmt::Display` or
// with `fmt::Debug`.
struct UnPrintable(i32);

// `derive` attribute automatically creates printing implementation
// for `fmt::Debug`.
#[derive(Debug)]
struct DebugPrintable(i32);

// Examples:
#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Wrap(Structure);

// Pretty printing:
#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}


fn main () {
    // `{:?}` automatically print `std` library types.
    // `{:?}` is like `{}`
    println!("{:?} months in a year.", 12);
    println!("{1:?} {0:?} is the {actor:?} name.",
             "Slater",
             "Christian",
             actor="actor's");

    // Printing a structs:
    println!("Now {:?} will print!", Structure(3));
    println!("Now {:?} will print!", Wrap(Structure(7)));

    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // Pretty print
    println!("{:#?}", peter);
}
