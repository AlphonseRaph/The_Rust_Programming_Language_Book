// using enum to store different data types from a vector
enum SpreadsheetCell {
Int((i32)),
Float(f64),
Text(String),
}


fn main() {
    // create an empty vector of i32s
    let _v: Vec<i32> = Vec::new();
    // create a vector with initial values
    let _v = vec![1, 2, 3, 4, 5];

    // Updating a Vector
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    // Reading Elements of a Vector
    // This element is best used when you are sure the index is valid, otherwise it will panic and crash your program.
    let third: &i32 = &v[2];
    println!("The third element is {}", third);
    // This element is best used when accessing  an element beyond the range of the vector may happen occassionally under normal circumstances,
    // it returns an Option enum that you can handle with a match expression.
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    // When the program has a valid reference, the borrow checker enforces
    // the ownership and borrowing rules

    let mut _v = vec![1, 2, 3, 4, 5];
    let first = &_v[0];
    // v.push(6); // This line would cause a compile-time error because it tries to modify the vector while we have an immutable reference to one of its elements.
    /*
     * WHY THIS FAILS: Vectors store elements contiguously in memory.
     * If `v.push(6)` exceeds the vector's current capacity, Rust must allocate a new,
     * larger memory block and move all existing elements into it.
     * If that happens, our `first` reference would be left pointing to the old,
     * deallocated memory (a dangling pointer). The borrow checker prevents this!
     */
    println!("The first element is: {first}");

    // Iterating Over a Vector
    /*
     * BORROW CHECKER AT WORK:
     * The `for` loop holds a reference to the entire vector for the duration of the loop.
     * You cannot insert or remove elements inside the loop body, because changing
     * the vector's size requires a mutable borrow of the whole vector. This would
     * conflict with the loop's active references and could potentially reallocate
     * memory, invalidating the items currently being iterated over.
     */
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }
    // To change the value of each element, we need to iterate over mutable references to the elements.
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50; // we need to dereference i to change the value inside the vector
    }

    //Using vectors to store enums which is currently holding different data types from a spreadsheet
    // Only works if you know all the possible types that the spreadsheet can hold.
    let _row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
    ];

    // Dropping a Vector
    {
    let v = vec![1, 2, 3, 4];
    // do stuff with v
    } // <- v goes out of scope and is freed here

    // String
    // A String is a wrapper over a vector of bytes that are guaranteed to be valid UTF-8.
    // create a new empty string
    let mut s = String::new();

    // Often we'll have some initial data that we want to convert to a String. To do that, 
    // we can use the to_string method provided by the ToString trait,
    // which is implemented for any type that implements the Display trait.
    // Both string literals and the String type implement Display, so we can call to_string on them.
    let data = "initial contents";
    let s = data.to_string();
    // the method also works on a literal directly:
    let s = "initial contents".to_string();

    // We can String::from function to create a String from a string literal:
    let s = String::from("initial contents");

    // Strings are UTF-8 encoded, so they can contain any valid UTF-8 character, including emojis and characters from non-Latin scripts.
    let hello = String::from("السلام عليكم");
    println!("{hello}");
    let hello = String::from("Dobrý den");
    println!("{hello}");
    let hello = String::from("Dobrý den");
    println!("{hello}");
    let hello = String::from("你好");
    println!("{hello}");
    let hello = String::from("안녕하세요");
    println!("{hello}");
    let hello = String::from("こんにちは");
    println!("{hello}");
    let hello = String::from("Olá");
    println!("{hello}");
    let hello = String::from("Здравствуйте");
    println!("{hello}");
    let hello = String::from("Hola");
    println!("{hello}");

    // Updating a String
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{s}");

    // The push_str method does not take ownership of the parameter, so we can still use s2 after appending it to s1.
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");

    // The push method takes a single character as a parameter and adds it to the end of the String.
    let mut s = String::from("lo");
    s.push('l');
    println!("{s}");

    // + operator can also be used to concatenate strings, but it takes ownership of the left operand and borrows the right operand.
    // it uses the add method, which is defined like this: fn add(self, s: &str) -> String. 
    // The first parameter is self, which means the add method takes ownership of the string on the left-hand side of the + operator. 
    // The second parameter is a string slice, which means it borrows the string on the right-hand side of the + operator.
    // The add method then concatenates the two strings and returns a new String.
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    println!("{s3}");

    // Using + with multiple strings is a bit more complicated because of the ownership rules.
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("{s}");

    // A more readable way to concatenate multiple strings is to use the format! macro,
    //  which is similar to println! but returns a String instead of printing it.
    // The format! macro does not take ownership of any of its parameters, so we can still use s1, s2, and s3 after concatenating them.
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{s1}-{s2}-{s3}");
    println!("{s}");

    // This causes an error because Rust strings does not support indexing. Rust strings are not arrays of bytes, but rather a collection of UTF-8 encoded characters.
    // let s1 = String::from("hi");
    // let h = s1[0];

    /*
    * WHY DIRECT STRING INDEXING (e.g., &s[0]) FAILS IN RUST:
    * A Rust String is secretly a Vec<u8> (a vector of raw bytes) encoded in UTF-8.
    * Because UTF-8 characters vary in size (1 to 4 bytes each), a single index 
    * points to a specific byte, not necessarily a valid, complete character.
    * Returning a raw byte fragment instead of a letter causes subtle bugs, 
    * especially with non-Latin text. To guarantee safety, Rust refuses to 
    * compile direct indexing and forces you to be explicit (e.g., using `.chars()`).
    */

    /*
    * WHY NO STRING INDEXING? (Part 2: Linguistics & Performance)
    * * 1. Complexity: Strings have 3 layers (raw bytes, Unicode chars, and human 
    * grapheme clusters). An index doesn't know which one you want.
    * 2. Performance: Indexing syntax `[i]` implies instant, O(1) performance. 
    * Because UTF-8 characters are variable-length, Rust would have to scan 
    * the string from the beginning (O(N) time) to find the right character. 
    * Rust blocks indexing to prevent hiding this performance penalty.
    */

    /*
    * DANGERS OF STRING SLICING:
    * You can slice strings using byte ranges (e.g., &s[0..4]), but it is risky.
    * The range targets raw bytes, not logical characters. If your range cuts 
    * through the middle of a multi-byte character (like taking only the first byte 
    * of a 2-byte emoji or letter), your program will PANIC and crash at runtime. 
    * Only use range slicing if you are absolutely certain of the byte boundaries!
    */

    /*
    * ITERATING OVER STRINGS IN RUST
    * Because strings are UTF-8 encoded byte vectors, Rust requires you to 
    * explicitly state how you want to read the data during iteration:
    *
    * 1. `.chars()` -> Iterates over Unicode scalar values (chars). 
    * Use this for standard text processing where you need actual letters.
    *
    * 2. `.bytes()` -> Iterates over raw `u8` bytes. 
    * Use this for low-level data manipulation. Be aware that multi-byte 
    * characters will produce multiple loop iterations.
    *
    * (Note: True human-readable letters, or grapheme clusters, are complex 
    * and require importing an external crate from crates.io).
    */

    let text = "Зд"; // A 2-character string that takes 4 bytes in memory

    // Iterating over characters
    println!("--- Characters ---");
    for c in text.chars() {
        println!("Char: {c}");
    }

    // Iterating over raw bytes
    println!("--- Bytes ---");
    for b in text.bytes() {
        println!("Byte: {b}");
    }


    // Hash Maps: Are also known as hash tables, dictionaries, or associative arrays in other programming languages.
    // They store a mapping of keys to values, where each key is unique and maps to exactly one value.
    // Hash maps are implemented using a hashing algorithm that converts the key into an index in an underlying array,
    // allowing for fast retrieval of values based on their keys.
    // They store data in the heap, each key should be the same type and each value should be the same type.
    // if you insert a key that already exists in the hash map, the old value will be overwritten with the new value.
    use std::collections::HashMap;
    // Creating a Hash Map
    let mut scores = HashMap::new();
    // Inserting Key-Value Pairs
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);


    // Accessing Values in a Hash Map
    let team_name = String::from("Blue");
    /*
    * HASHMAP LOOKUP CHAIN EXPLAINED:
    * 1. `.get(&key)`: Looks for the key. Returns an `Option<&V>` (a reference to 
    * the value, so we don't take ownership). If missing, returns `None`.
    * 2. `.copied()`: Converts the `Option<&i32>` into an `Option<i32>` by copying 
    * the underlying value. Now we have an owned integer, not a reference.
    * 3. `.unwrap_or(0)`: Safely opens the Option. If it's `Some(value)`, it gives 
    * us the value. If it's `None`, it provides our fallback value (0).
    */
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("The score of team {team_name} is: {score}");

    // Iterating Over a Hash Map
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // Hash Maps and Ownership
    // For types that implement the Copy trait, like i32, the values are copied into the hash map.
    // For owned types like String, the values will be moved into the hash map.
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point

    // Updating a Hash Map
    // Overwriting a Value
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    println!("{scores:?}");
    // Adding a Key and Value Only If a Key Isn’t Present
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    // or_insert returns a mutable reference to the value for the specified key:
    scores.entry(String::from("Yellow")).or_insert(50); // This will insert the key "Yellow" with the value 50 because "Yellow" is not already in the hash map.
    scores.entry(String::from("Blue")).or_insert(50); // This will not change the value for "Blue" because "Blue" is already in the hash map with the value 10.
    println!("{scores:?}");
    // Updating a Value Based on the Old Value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() { // Iterates over words, safely ignoring extra spaces, tabs, and newlines.
    let count = map.entry(word).or_insert(0); // Finds the key or inserts 0 if missing. Returns a mutable reference (&mut i32) to the value.
    *count += 1; // Dereferences (*) the reference to modify the actual integer, incrementing the count.
    }
    println!("{map:?}"); // Prints the map using debug formatter (:?). Hash map output order is random.

    /*
    * HASHMAP SECURITY VS. PERFORMANCE:
    * By default, Rust's HashMap uses the 'SipHash' algorithm. 
    * - Pros: It is cryptographically resistant to HashDoS (Denial of Service) attacks.
    * - Cons: It is not the absolute fastest hashing algorithm available.
    * * If you profile your code and find HashMap inserts/lookups are a bottleneck, 
    * you can override this default by specifying a faster, non-secure hasher 
    * (like 'rustc-hash' or 'ahash').
 */


}