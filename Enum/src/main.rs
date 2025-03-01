// Enum is a custom data type that allows you to define a type by enumerating its possible values.
// Enums in Rust are similar to those in other languages, such as C, 
// but have some additional features.
// An enum is defined using the enum keyword, followed by the name of the enum and a block of curly braces.
// Each value in the enum is called a variant and is separated by a comma.

fn main() {
    // Define an enum to represent IP address types
    enum IpAddrKind {
        V4,
        V6,
    }

    // Create instances of the enum variants
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    // Define a function that takes an IpAddrKind and prints the type
    fn route(ip_kind: IpAddrKind) {
        /*  match: it allows you to compare a value against multiple 
        patterns and execute code based on which pattern matches. 
        */

        match ip_kind {
            IpAddrKind::V4 => println!("V4"),
            IpAddrKind::V6 => println!("V6"),
        }
    }

    // Call the function with different enum values
    route(four); // Output: "V4"
    route(six);  // Output: "V6"

    // Using Structs 
    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    let _home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let _loop_back = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    // Using Enums with Data
    enum IpAddrData {
        V4(String),
        V6(String),
    }

    let _home1 = IpAddrData::V4(String::from("127.0.0.1"));
    let _loop_back1 = IpAddrData::V6(String::from("::1"));


    // Enhanced Enum with Data
    enum  IpAddrData1 {
        V4(u8, u8, u8, u8),
        V6(String), 
    }

    let _home2 = IpAddrData1::V4(127, 0, 0, 1);
    let _loop_back2 = IpAddrData1::V6(String::from("::1"));
}
