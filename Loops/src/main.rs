// Repetition with Loops:
// Loops are used to execute a block of code multiple times until a certain condition is met.
// While are used when you don't know how many times you want to loop.
// For are used when you know how many times you want to loop.
fn main() {
    // Loop keyword
    // loop{
    //     println!("Loop forever");
    // }

    let mut counter = 0;
    let result = loop{
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is: {}", result);

    // Loop Labels to Disambiguate Between Multiple  Loops
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");


    // While Loop
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");

    // Looping Through a Collection with for
    let a = [10, 20, 30, 40, 50];
    let b = ["Hello", "World", "Rust"];
    for letter in b {
        println!("The word is: {}", letter);
    }
    for element in a {
        println!("The value is: {}", element);
    }
}
