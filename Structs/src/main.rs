// Structs are used to create custom data types
// Structs are similar to tuples
// Structs can have named fields
// Structs can have methods
fn main() {
    // tuple
    let _rect = (30, 50);
   

    // struct

    struct Book {
        title: String,
        author: String,
        pages: u32,
        price: f64,
        availability: bool,
    }

    struct User{
        name: String,
        email: String,
        age: u32,
        active: bool,
        sign_in_count: u64,
    }

    let mut user1 = User{
        name: String::from("John Doe"),
        email: String::from("johndoe@gmail.com"),
        age: 32,
        active: true,
        sign_in_count: 1
    };

    user1.email = String::from("another@gmail.com");

    println!("User email is: {}", user1.email);

    // Return a struct from a function


    
    // fn build_user(email: String, username: String) -> User{
    //     User {
    //         active: true,
    //         email,
    //         name,
    //         age: 32,
    //         sign_in_count: 1,
    //     }
    // }

    // Create instances from other instances
    let user2 = User{
        email: String::from("johndoe@gmail.com"),
        // ..user1 means use the rest of the fields from user1
        ..user1
    };

    // Tuple Structs
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0,0,0);
    let white = Color(255,255,255);

    // Unit-like structs
    // Structs with no fields are called unit-like structs
    // They are useful when you need to implement a trait on a type 
    // but don't have any data to store in the type itself

    struct AlwaysEqual;
    let subject = AlwaysEqual;
    
}
