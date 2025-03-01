// Compound Data Types
// arrays, tuples, slices, and strings (slice string)

fn main() {
    // Arrays
    let numbers:[i32; 4] = [1,2,3,4];
    println!("Number Array: {:?}", numbers);

    let fruits: [&str; 3] = ["apple", "banana", "mango"];
    println!("Array of fruits {:?}", fruits);
    println!("Array of fruits {}", fruits[0]);
    println!("Array of fruits {}", fruits[1]);
    println!("Array of fruits {}", fruits[2]);

    // Tuples
    // Strings
    let human: (String, i32, bool) = ("Alice".to_string(), 30, true);
    println!("Human Tuple: {:?}", human);


    let my_mix_tuple:(&str, i32, bool, [i32; 5]) = ("Kratos", 23, true, [1,2,3,4,5]);
    println!("My Mix Tuple: {:?}", my_mix_tuple);
    let my_mix_tuple= ("Kratos", 23, true, [1,2,3,4,5]);
    println!("My Mix Tuple: {:?}", my_mix_tuple);

    // Slices: [1,2,3,4,5]
    let number_slices: &[i32] = &[1,2,3,4,5];
    println!("Number Slice: {:?}", number_slices);

    let animal_slices: &[&str] = &["Lion", "Elephant", "Goat"];
    println!("Animal Slice: {:?}", animal_slices);

    let book_slices: &[String] = &["Bible".to_string(), "Quran".to_string(), "IT".to_string()];
    println!("Book Slice: {:?}", book_slices);

    // Strings Vs String Slices (&str)
    // Strings [growable, mutable, owned string type ]
    let mut stone_cold: String = String::from("Heaven, ");
    println!("Stone Cold Says: {}", stone_cold);
    stone_cold.push_str("Yeah!");
    println!("Stone Cold Says: {}", stone_cold);

    // B- &str (String Slice)
    let string: String = String::from("Hello, World");
    let slice: &str = &string[0..5];
    println!("Slice Value: {}", slice);
}

