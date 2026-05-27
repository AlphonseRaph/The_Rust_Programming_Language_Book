fn main() {
    let s1 = String::from("hello");
    // We pass a reference to s1, and the function calculates the length of the string without taking ownership of it.
    let len = calculate_length(&s1);
    println!("The length of '{s1}' is {len}.");

// We create a mutable String and pass a mutable reference to the change function, which modifies the string in place.
let mut s = String::from("hello");
 { 
change(&mut s);

// we can use curly brackets to create a new scope, allowing for multiple mutable references
let mut s = String::from("hello");
{
let r1 = &mut s;
} // r1 goes out of scope here, so we can make a new reference with no problems
let r2 = &mut s;
}


/*
    The scopes of the immutable references r1 and r2 end after the
    println! where they are last used, which is before the mutable
    reference r3 is created. These scopes don’t overlap, so this
    code is allowed: The compiler can tell that the reference is no
    longer being used at a point before the end of the scope.
*/
let mut s = String::from("hello");
let r1 = &s; // no problem
let r2 = &s; // no problem
println!("{r1} and {r2}");
// variables r1 and r2 will not be used after this point
let r3 = &mut s; // no problem
println!("{r3}");


// string slices are references to a portion of a String. They allow you to reference a contiguous sequence of characters in a String without taking ownership of the whole String.
let s = String::from("hello world");
let hello = &s[0..5];
let world = &s[6..11];
println!("{hello} {world}");

// Stating at index 0 has two syntactic sugar forms: &s[..2] is the same as &s[0..2]
let s = String::from("hello");
let slice = &s[0..2];
println!("{slice}"); 
let slice = &s[..2];
println!("{slice}");

// Stating the length of the string has two syntactic sugar forms: &s[3..len] is the same as &s[3..]
let s = String::from("hello");
let len = s.len();
let slice = &s[3..len];
println!("{slice}");
let slice = &s[3..];
println!("{slice}");

// Stating the whole string has two syntactic sugar forms: &s[0..len] is the same as &s[..]
let s = String::from("hello");
let len = s.len();
let slice = &s[0..len];
println!("{slice}");
let slice = &s[..];
println!("{slice}");

let word = first_word(&s);
println!("{word}");
// s.clear(); // error! // We cannot clear the string because we have an immutable reference to it (the variable word). 
// If we were to clear the string, the reference would point to invalid data, which is not allowed in Rust. 
// The compiler prevents this by giving us an error if we try to modify the string while we have an immutable reference to it.
println!("the first word is: {word}");



{
    // ==========================================
    // 1. Working with Heap-Allocated Strings
    // ==========================================

    // The first half of the code creates a dynamically allocated string on the heap:
    let my_string = String::from("hello world");

    // You are taking a specific, partial chunk of the String (the word "hello "). 
    // Slicing a String using brackets instantly produces an &str.
    let word1 = first_word(&my_string[0..6]);

    // The .. syntax means "the whole thing." You are explicitly taking a slice 
    // of the entire String, which also produces an &str.
    let word2 = first_word(&my_string[..]);

    // You are passing a direct reference to the String. As we covered previously, 
    // Rust's Deref Coercion steps in here, invisibly converting the &String into an &str for you.
    let word3 = first_word(&my_string);


    // ==========================================
    // 2. Working with String Literals
    // ==========================================

    // The second half of the code uses a hardcoded string literal. 
    // Remember that the type of this variable is already &str.
    let my_string_literal = "hello world";

    // Just like with the String, you can slice a literal to get a smaller &str (just the word "hello ").
    let word4 = first_word(&my_string_literal[0..6]);

    // You are explicitly slicing the entire literal. It is slightly redundant, but perfectly valid Rust.
    let word5 = first_word(&my_string_literal[..]);

    // Notice there is no & symbol here! Because my_string_literal is already of type &str, 
    // and the function asks for an &str, the types match perfectly. You can just pass the variable directly.
    let word6 = first_word(my_string_literal);
}

// Other slices
let a = [1, 2, 3, 4, 5];
let slice = &a[1..3];
assert_eq!(slice, &[2, 3]);


}


fn calculate_length(s: &String) -> usize { // The parameter s is a reference to a String. We can use it to access the data without taking ownership of it.
    s.len()  
}

// The parameter some_string is a mutable reference to a String. We can modify the string through this reference.
fn change(some_string: &mut String) { 
some_string.push_str(", world");
println!("{some_string}");
}

// The function first_word takes a reference to a String and returns a string slice (&str) that represents the first word in the string. 
// It does this by iterating through the bytes of the string and looking for the first space character. 
// If it finds a space, it returns a slice of the string from the beginning to the index of the space.
// If it doesn't find a space, it returns a slice of the entire string.
fn first_word(s: &str) -> &str { // we pass in &str instead of &String because we want to be able to use this function with string literals as well as String instances.
let bytes = s.as_bytes();
for (i, &item) in bytes.iter().enumerate() {
if item == b' ' {
return &s[0..i];
}
}
&s[..]
}