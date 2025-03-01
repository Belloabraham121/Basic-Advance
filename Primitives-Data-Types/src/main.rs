// Primitive data types
// int, float, bool, char

// Integer
// Rust has signed (+ and - ) and unsigned
//integer (only+) types of different sizes.

fn main() {
    let x: i32 = 42;
    let y: u64 = 100;
    println!("Signed Integer: {}", x);
    println!("Unsigned Integer: {}", y);

    //diff bet !32 (32 bits) and i64(64 bits)
    // range :
    // i32 - 2147483647 (2^31 or 2^-31)
    // i64 - 9223372036854775607

    let e: i32 = 2147483647;
    let i: i64 = 9223372036854775607;
    println!("Maximum value for {}", e);
    println!("Maximum value for {}", i);

    // Floats [Floating Point Types]
    // f32 f64

    let pi: f64 = 3.14;
    println!("Value of pi {}", pi);

    // Boolean Values: true, false
    let is_snowing: bool = true;
    println!("Is is snowing? {}", is_snowing);

    // Character Type = char

    let letter: char = 'a';
    println!("First letter of the alphabet: {}", letter);
}

