// Shadowing 
// Shadowing is different from marking a variable as mut, 
// because we’ll get a compile-time error if we accidentally try to reassign to this variable without using the let keyword. 
// By using let, we can perform a few transformations on a value but have the variable be immutable after those transformations have been completed.
fn main() {
    let x = 5; // result is 5

    let x = x + 1; //  result is 6

    let spaces = "   "; // result is "   "
    let spaces = spaces.len(); // result is 3

    {
        let x = x * 2; // result is 12
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
    println!("The value of spaces is: {spaces}");
}