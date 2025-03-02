// Approach 1
// Option<T> is an enum defined by the standard library
// It is used to represent the presence or absence of a value
// It is a generic type, so it can hold values of any type
// It has two variants: Some(T) and None
// Some(T) represents the presence of a value of type T
// None represents the absence of a value
// enum Option<T> { // Defining the generic option type
//     Some(T),    // Represents a value
//     None,       // Represents no value
// }

fn divide_option(x: f64, y: f64) -> Option<f64> {
    if y == 0.0 {
        None
    } else {
        Some(x / y)
    }
}

// Approach 2
// The Result<T, E> enum is defined by the standard library
// It is used to represent the result of an operation that can fail
// It is a generic type, so it can hold values of any type
// It has two variants: Ok(T) and Err(E)
// Ok(T) represents a successful result with a value of type T
// Err(E) represents a failed result with an error of type E
// enum Result<T, E> { // Defining the generic result type
//     Ok(T),  // Represents a successful result
//     Err(E), // Represents a failed result
// }

fn divide_result(x: f64, y: f64) -> Result<f64, String> {
    if y == 0.0 {
        Err("Division by zero".to_string())
    } else {
        Ok(x / y)
    }
}


fn main() {
    let result = divide_option(10.0, 2.0);
    match result {
        Some(value) => println!("Result: {}", value),
        None => println!("Error: Division by zero"),
    }

    let result1 = divide_result(10.0, 0.0);
    match result1 {
        Ok(value) => println!("Result: {}", value),
        Err(error) => println!("Error: {}", error),
    }



}
