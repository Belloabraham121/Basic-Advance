// If Else [If Expression] [Else Expression]

fn main() {

    
    let age:u16 = 18;
    if age >= 18 {
        println!("You can drive a car")
    }else {
        println!("You can't drive a car")
    }

    // Multiple conditiions with else if
    let number = 3;
    if number % 4 == 0 {
        println!("Number is divisible by 4")
    }else if number % 3 == 0 {
        println!("Number is divisible by 3")
    }else if number % 2 == 0 {
        println!("Number is divisible by 2")
    }else {
        println!("Number is not divisible by 2, 3, 4")
    }

    // Using if in let statement
    let condition = true;
    let number = if condition {
        5
    }else {
        6
    };
    println!("The value of number is: {}", number);
}
