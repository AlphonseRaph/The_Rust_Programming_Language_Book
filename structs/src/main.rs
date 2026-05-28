// Structs are custom data types that let you name and package together multiple related values that make up a meaningful group.
struct User {
active: bool,
username: String,
email: String,
sign_in_count: u64,
}
// We can also define unit-like structs that don’t have any fields. These are useful when you need to implement a trait on some type but don’t have any data that you want to store in the type itself. Here's an example of a unit-like struct:
struct AlwaysEqual;

fn main() {
// To create an instance of a struct, we specify the name of the struct and then provide values for each of the fields in the struct, like this:
let user1 = User {
active: true,
username: String::from("someusername123"),
email: String::from("someone@example.com"),
sign_in_count: 1,
};
println!("User1 email: {}, username: {}", user1.email, user1.username);


// We can also make the instance mutable, which allows us to change the value of one or more fields in that instance. To do that, we add mut before the name of the instance, like this:
let mut user1 = User {
active: true,
username: String::from("someusername123"),
email: String::from("someone@example.com"),
sign_in_count: 1,
};
user1.email = String::from("anotheremail@example.com"); // Using . to access the field and change its value

// Returning an instance of a struct from a function is also common. We can write a function that takes some parameters and then constructs and returns an instance of the struct. Here's an example:
let user2 = build_user(String::from("user2@example.com"), String::from("user2-username123"));
println!("User2 email: {}, username: {}", user2.email, user2.username);

// Without using struct update syntax, we would have to specify the value for each field in the new instance. 
let user3 = User {
active: user1.active,
username: user1.username,
email: String::from("user3-email@example.com"),
sign_in_count: user1.sign_in_count,
};
println!("User3 email: {}, username: {}", user3.email, user3.username);

// Using struct update syntax, we can specify only the fields that we want to change and then use .. to specify that the remaining fields should have the same values as the fields in the given instance. Here's how we can do that:
let user4 = User {
email: String::from("another@example.com"),
// Struct update syntax (`..user1`) only pulls missing fields.
// We define `username` here to prevent Rust from trying to reuse `user1`'s moved string.
username: String::from("user4_own_name"),
..user1
};
println!("User4 email: {}, username: {}", user4.email, user4.username);

// Tuple structs are a different way to define structs that look similar to tuples, but have named types. They are useful when you want to give the tuple a name and make the tuple fields more meaningful. Here's an example of how to define and use tuple structs:
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
fn main() {
let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);
}



let subject = AlwaysEqual;


}

// We can also write a function that returns an instance of the struct. Here's an example of a function that builds a User struct and returns it:
// fn build_user(email: String, username: String) -> User {
// User {
// active: true,
// username: username,
// email: email,
// sign_in_count: 1,
// }
// }

// No repetition of field names in the function body when the parameter names are the same as the field names. We can use field init shorthand syntax to make this more concise, like this:
fn build_user(email: String, username: String) -> User {
User {
active: true,
username,
email,
sign_in_count: 1,
}

}
