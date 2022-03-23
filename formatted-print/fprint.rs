/*
 * 
 *  format!: write formatted text to String
 *  print!: same as format! but the text is printed to the console (io::stdout).
 *  println!: same as print! but a newline is appended.
 *  eprint!: same as format! but the text is printed to the standard error (io::stderr).
 *  eprintln!: same as eprint!but a newline is appended.
 */

fn main() {
    // `{}` replaced by arg and stringified
    println!("{} forty-two", 42);

    // Plain decimal defaults to i32.
    // Force type: <num><type>
    // e.g. `42i64`
    
    // Positional Arguments:
    println!("{0}{1}{1}{0}", "A", "B");

    // Named Arguments:
    println!("{a} {b} {c}",
             a="a",
             b="b",
             c="c");

    // Special formatting (after `:`):
    println!("{} vs. {:b}", 15, 15);

    // Right align
    // See std::fmt for more
    println!("{number:>width$}", number=42, width=10);

    // Pad with zeroes
    // See std::fmt for more
    println!("{number:0>width$}", number=42, width=10);

    // Rust checks for expected # of args
    
    // Disable compiler lint warning about unused functions
    #[allow(dead_code)]
    // Struct `Structure` which contains an `i32`.
    struct Structure(i32);

    // println!("This struct `{}` won't print...", Structure(3));
    
    // Decimal place specification:
    println!("Pi is roughly {:.3}", pi=3.141592);
}
