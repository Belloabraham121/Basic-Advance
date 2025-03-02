// Vectors
// 

fn main() {
    
    // let _v: Vec<i32> = Vec::new();
    // let _the_vec: Vec<i32> = vec![1, 2, 3];
    
    // Macro to create a vector of numbers
    let mut _v: Vec<i32> = Vec::new();
    let mut _v = vec![1,2,3,4];
    _v.push(5);
    _v.push(6);
    _v.push(7);
    _v.push(8);
    println!("{:?}", _v);

    // Accessing elements in a vector
    let _vec: Vec<i32> = Vec::new();
    let _vec = vec![1,2,3,4,5];
    let _third: &i32 = &_vec[2];
    println!("The third element is {}", _third);

    let third = _vec.get(2);
    match third {
        Some(value) => println!("The third element from a GET method is {}", value),
        None => println!("There is no third element"),
    }


    // -------------------UTF-8-------------------
    // 1
    let _s = "Whatever".to_string();
    // 2
    let _s = String::from("Whatever");
    // 3 Mutate the variable [push to it]
    let mut s = String::from("Hello, ");
    // s.push("1"); If I'm pushing a character
    s.push_str("world!");
    println!("{}", s);

    // If I want to concatenate two strings
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1 has been moved here and can no longer be used
    println!("{}", s3);

    // Formatting strings
    let s4 = String::from("tic");
    let s5 = String::from("tac");
    let s6 = String::from("toe");

    let s = format!("{s4}-{s5}-{s6}");
    println!("{}", s);  
}
