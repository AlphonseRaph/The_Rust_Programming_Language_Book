fn main() {
    // s is not valid here, since it's not yet declared
    // VARIABLE SCOPE

    let _s = "hello"; // s is valid from this point forward (s is a stirng literal, immutable and stored in stack)
    // do stuff with s

    let mut s = String::from("hello"); // string type (can be mutated and is stored on the stack)
    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{s}"); // this will print `hello, world!`

    // scope example using String instead of string literal
    {
        let _s = String::from("hello"); // s is valid from this point forward
        // do stuff with s
    } // this scope is now over, and s is no
    // longer valid

    let s1 = String::from("hello");
    let s2 = s1;
    // println!("{s1}, world!"); This will cause an error, s1 will move to s2 , s1 is now invalid
    println!("{s2}, world!");

    // to deeply copy a value, we can use the clone method.
    // This will allocate enough memory for the data on the heap and copy the data over
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {s1}, s2 = {s2}");

    // when you assign a completely new value to an existing variable, Rust will call drop and free the original
    // value's memory immediately
    let mut _z = String::from("hello");
    _z = String::from("ahoy");
    println!("{_z}, world!");

    // Copying stack data is very fast, so types that have a known size at compile time can be copied rather than moved.
    let x = 5;
    let y = x;
    println!("x = {x}, y = {y}");

    {
        let s = String::from("hello"); // s comes into scope
        takes_ownership(s); // s's value moves into the function...
        // ... and so is no longer valid here
        let x = 5; // x comes into scope
        makes_copy(x); // because i32 implements the Copy trait,
        // x does NOT move into the function,
        // so it's okay to use x afterward
    } // Here, x goes out of scope, then s. However, because s's value was moved,
    // nothing special happens.
    fn takes_ownership(some_string: String) {
        // some_string comes into scope
        println!("{some_string}");
    } // Here, some_string goes out of scope and `drop` is called. The backing
    // memory is freed.
    fn makes_copy(some_integer: i32) {
        // some_integer comes into scope
        println!("{some_integer}");
    } // Here, some_integer goes out of scope. Nothing special happens.

    {
        let s1 = gives_ownership(); // gives_ownership moves its return value into s1
        let s2 = String::from("hello"); // s2 comes into scope
        let s3 = takes_and_gives_back(s2); // s2 is moved into
        // takes_and_gives_back, which also
        // moves its return value into s3
    } // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing happens. s1 goes out of scope and is dropped.
    fn gives_ownership() -> String {
        // gives_ownership will move its return value into the function that calls it
        let some_string = String::from("yours"); // some_string comes into scope
        some_string // some_string is returned and moves out to the calling function
    }
    // this function takes a String and returns a String
    fn takes_and_gives_back(a_string: String) -> String {
        // a_string comes into scope
        a_string // a_string is returned and moves out to the calling function
    }
} // this scope is now over, and s is no longer valid ()
