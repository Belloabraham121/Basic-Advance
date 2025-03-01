// Functions
// fn main is how Entry Point
// amy function / variables should be written in snake case
// : hello_world

fn main() {
    hello_world();
    tell_height(32);
    human_id("John Doe", 32, 193.3);
    let _x = {
        let price = 5;
        let qty = 10;
        price * qty
    };
    println!("Result is {}", _x);

    let y = add(2,4);
    println!("Value of y is : {}", y);
    println!("Value from function 'add' is: {}.", add(2,4));

    // Calling the BMI function
    let weight = 70.0;
    let height = 1.82;
    let bmi = calculate_bmi(weight, height);
    println!("Your BMI is: {:.5}", bmi);

    // Calculate Area of Rectangle
    println!("Area of Rectangle is: {}", calculate_area_of_rectangle(5, 10));

    let x: i32 = 10;
    let y: f64 = 3.5;
    println!("{}", x as f64 + y);

    println!("{}", check_even(4)); // true
    println!("{}", check_even(7)); // false


    // Find the even numbers
    let numbers = [1, 2, 3, 4, 5, 6];
    print_even_numbers(numbers);

    // Find the largest number
    let numbers = [3, 7, 2, 9, 5];
    let max_num = largest_num(numbers);
    println!("The largest number is {}", max_num);

    // Count the even and odd numbers
    let numbers = [4, 7, 2, 9, 10, 5, 8];
    count_odd_even(numbers);
}


fn check_even(n: i32) -> bool {
    n % 2 == 0
}

// Hoisting - can call function anywhere in your code
fn hello_world(){
    println!("Hello, World");
}

// you can insert input values

fn tell_height(height: i32){
    println!("My height is {}", height);
}

// you cna insert more than one value

fn human_id(name: &str, age: u32, height: f32){
    println!("My name is {}, I am {} years old, and height is {} cm.", name, age, height);
}

fn add(a: i32, b: i32) -> i32{
    a + b
}

// Expressions and Statements
// Expressions: Anything that returns a values.
// Statement: Anything that does not return a values
// example: let x = 10;


// Final Example
// BMI = height(kg)/height(m)^2

fn calculate_bmi(weight_kg: f64, height_m: f64) -> f64{
    weight_kg / (height_m * height_m)
}

fn calculate_area_of_rectangle(width: i32, height: i32) -> i32{
    width * height
}

fn print_even_numbers(arr: [i32; 6]){
    for num in arr {
        if num % 2 == 0 {
            println!("{}", num);
        }
    }
}

fn largest_num(arr: [i32; 5]) -> i32{

    let mut largest = arr[0];
    for num in arr {
        if num > largest{
            largest = num;
        }
    }
    largest
}

fn count_odd_even(arr: [i32; 7]){
    let mut even_count = 0;
    let mut odd_count = 0;

    for num in arr {
        if num % 2 == 0{
            even_count += 1;
        }
        else {
            odd_count += 1;
        }
    }

    println!("Even numbers: {}", even_count);
    println!("Odd numbers: {}", odd_count);
}