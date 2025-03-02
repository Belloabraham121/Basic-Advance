// Hash Maps
// Hash maps are a particular implementation of a more general data structure called a map. 
// A map is a collection of key-value pairs. 
// In Rust, hash maps are implemented using a collection called HashMap. 
// You can create a new, empty HashMap using a static method called new.


fn main() {
    //----------------HASH MAPS----------------
    use std::collections::HashMap;


    let mut scores = HashMap::new();
    /*  The .insert() method is used to add key-value pairs to the HashMap.
    String::from("Blue") and String::from("Yellow") are used as keys, 
    and 10 and 50 are their respective values.
    */

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");

    /*  .get(&team_name) retrieves the value associated with the key "Blue".
    Since .get() returns an Option<&i32>, the .copied() method is used to convert &i32 (a reference) into i32 (a value).
    .unwrap_or(0) ensures that if the key does not exist, it returns 0 instead of None.
    */
    
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("Score of {} is {}", team_name, score);

    // Iterating over a HashMap
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}
