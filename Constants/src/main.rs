// Constants are always immutable
// Constants are declared using the const keyword
// Constants needs to be annotated with a type

fn main() {

    let  x = 5;
    const Y: i32 = 10;
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", Y);
    println!("Hello, world!");
    println!("The value of PI is: {}", PI);
    println!("The value of THREE_HOURS_IN_SECONDS is: {}", THREE_HOURS_IN_SECONDS);
}

// Constants can be declared in any scope, including the global scope, 
// which makes them useful for values that many parts of code need to know about.

const PI: f32 = 3.14159;
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

