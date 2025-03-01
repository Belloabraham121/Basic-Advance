// Ownership, Borrowing and References

// Ownership
// ---------
// C, C++ -> Memory Management Control Issue
// Garbage Collector solve this issue, but created a new issue -> Slow Performance:
// [stopping/Resuming the program]
// What is Ownership?
// Every value has a single owner [ every variable has one value, and it is it's sole owner]
// Ownership Rules
// ---------
// 1- Each value in Rust has an owner.
// 2- There can only be one owner at a time.
// 3- When the owner goes out of scope, the value will be dropped.

// Example: Each value in Rust has a variable that's its owner.
fn main() {
    // Example: Each value in Rust has a variable that's its owner.

    let s1 = String::from("Rust");
    let len = calculate_length(&s1);
    println!("Length of '{}' is {}.", s1, len);

    // 2- There can only be one owner at a time.
    let s2 = String::from("Rust");
    let s3 = s2;
    println!("{}", s3);

    // 3- When the owner goes out of scope, the value will be dropped.
    let s4 = String::from("Rust");
    let _len = calculate_length(&s4);
    println!("Length of '{}' is {}.", s1, _len);

}
// s4 goes out of scope and its value will be dropped
// fn print_lost(s: &string){
//     println!("{}", &s4);
// }

fn calculate_length(s: &String) -> usize{
    s.len()
}