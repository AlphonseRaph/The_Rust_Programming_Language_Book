// Enums used to define a type by enumerating/listing its possible variants.
enum IpAddrKind {
    V4,
    V6,
}

// We can also define an enum that has data associated with each variant.
enum IpAddrString {
    V4(String),
    V6(String),
}

// We can also define an enum that has different types and amount of data associated with each variant.
enum IpAddress {
    V4(u8, u8, u8, u8),
    V6(String),
}

// How the standard library has defined the IpAddr enum:
// You can put any data you want in the enum variants, even enums.
struct Ipv4Addr {}
struct Ipv6Addr {}
enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

// Another example of an enum with different types and amount of data associated with each variant.
enum Message {
    Quit,                       // No data associated with this variant.
    Move { x: i32, y: i32 },    // Has named fields associated with this variant.
    Write(String),              // Has a single String associated with this variant.
    ChangeColor(i32, i32, i32), // Has three i32 values associated with this variant.
}

// enums is used when a value can be only one of several possible variants at any given time,
// in contrast to structs whih requires multiple values to be present at the same time. e.g username, email and active when creating a user struct
enum RequestStatus {
    Pending,                                    // OR
    Success(String),                            // OR (Can even hold data inside the variant!)
    Failed { error_code: u32, reason: String }, // OR (Can hold struct-like data!)
}

// We can also implement methods on enums, just like we do with structs.
impl RequestStatus {
    // A method for the enum
    fn print_status(&self) {
        // We use 'match' to check which variant it currently is
        match self {
            RequestStatus::Pending => println!("Still working..."),
            RequestStatus::Success(data) => println!("Done! Result: {}", data),
            RequestStatus::Failed { error_code, reason } => {
                println!("Error {}: {}", error_code, reason)
            }
        }
    }
}
// OR
impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        // match can use any type in the expression in this case we used type enum Coin, in contrast to if which requires a boolean expression.
        Coin::Penny => 1, // match arms which has two parts, pattern and code to run if the expression matches that pattern.
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn long_value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        } // We can also have multiple lines of code in the match arm but they must be enclosed in curly braces and the last expression is the value that will be returned from the arm. Comma is optional when using curly braces.
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

// We can also have match arms that have patterns with associated data, like the Quarter variant in the Coin_WithState enum.
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}
impl UsState {
    // Implementing a method on UsState to check whether it existed in a given year.
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819, // if UsState is Alabama, then it existed in the year if the year is greater than or equal to 1819, which is the year Alabama became a state.
            UsState::Alaska => year >= 1959, // if UsState is Alaska, then it existed in the year if the year is greater than or equal to 1959, which is the year Alaska became a state.
        }
    }
}

enum Coin_WithState {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
fn value_in_cents_in_state(coin: Coin_WithState) -> u8 {
    match coin {
        Coin_WithState::Penny => 1,
        Coin_WithState::Nickel => 5,
        Coin_WithState::Dime => 10,
        Coin_WithState::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}

// Using if let syntax to check if a coin is a quarter and if it is, we can also check the state of the quarter and return a message based on whether the state existed in 1900 or not. If the coin is not a quarter, we return None.
fn describe_state_quarter(coin: Coin_WithState) -> Option<String> {
    if let Coin_WithState::Quarter(state) = coin {
        if state.existed_in(1900) {
            Some(format!("{state:?} is pretty old, for America!"))
        } else {
            Some(format!("{state:?} is relatively new."))
        }
    } else {
        None
    }
}

// Another syntax to describe the state quarter using if let syntax, but is easier to follow.
fn describe_state_quarter_2(coin: Coin_WithState) -> Option<String> {
    let state = if let Coin_WithState::Quarter(state) = coin {
        state
    } else {
        return None;
    };
    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}

//  Using let else syntax to describe the state quarter, which is more concise and easier to read than the previous two versions.
fn describe_state_quarte_3r(coin: Coin_WithState) -> Option<String> {
    let Coin_WithState::Quarter(state) = coin else {
        return None;
    };
    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}

fn main() {
    // We can use the enum variants like this:
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    // We can also create instances of the IpAddr enum with associated data.
    let _home = IpAddrString::V4(String::from("127.0.0.1"));
    let _loopback = IpAddrString::V6(String::from("::1"));

    // We can also create instances of the IpAddress enum with different types and amount of associated data.
    let _home_ip_adress = IpAddress::V4(127, 0, 0, 1);
    let _loopback_ip_adress = IpAddress::V6(String::from("::1"));

    // Calling the route function with different variants of the enum.
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    //Using RequestStatus enum and calling the method on it.
    let my_request = RequestStatus::Success(String::from("Data loaded!"));

    // Calling the method using dot notation, just like a struct!
    my_request.print_status();

    // Using the Message enum and calling the method on it.
    let m = Message::Write(String::from("hello"));
    m.call();

    // Normal variable analogy of no box
    let _x: i8 = 5;
    // Using the Option built-in enum to represent a value that can be either something or nothing. Analogy of a box that can either be empty or contain something
    let _y: Option<i8> = Some(5); // Analogy of a box that contains the value 5
    let _z: Option<i8> = None; // Analogy of an empty box
    // We don't need need to include Option<T> in the prelude because it's so commonly used.
    let _some_number = Some(5);
    let _some_char = Some('e');
    // This causes an error, analogy of box when trying to add a something in a closed box, even if its not empty,
    // you have to open the box first. Rust forces you to open the box first, if it is not empty, add the two items,
    // if it is empty, then you can't add anything to it. This is a safety feature to prevent null pointer exceptions and other bugs related to null values in other languages.
    // let sum = x + y; // ERROR!

    // using value_in_cents_in_state function with the Coin_WithState enum which has a variant that holds data, in this case the Quarter variant holds a UsState value.
    value_in_cents_in_state(Coin_WithState::Quarter(UsState::Alaska));

    // Testing the plus_one function with different Option<i32> values.
    let five = Some(5);
    println!("{:?}", plus_one(five)); // This will print Some(6)
    let none = plus_one(None);
    println!("{:?}", none); // This will print None

    // Demonstrate using "other" to catch and execute some code for all other values that are not explicitly matched in the match expression.
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }

    // Using an underscore `_` to catch all other values that are not explicitly matched in the match expression, but we don't care about the value itself, we just want to execute some code for all other values.
    let dice_roll_2 = 9;
    match dice_roll_2 {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }

    // In this case nothing will happen for the other values, we just want to ignore them and not execute any code for them, so we can use an underscore `_` and an empty tuple `()` to indicate that we are ignoring those values and not executing any code for them.
    let dice_roll_3 = 9;
    match dice_roll_3 {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }

    // Verbose way:
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"), // if the value is Some (True in Option enum), we print the value inside the Some variant in the code for that arm.
        _ => (), // if the value is None (False in Option enum), we do nothing and just ignore it with an underscore `_` and an empty tuple `()`.
    }

    // Concise way using if let syntax:
    let config_max_2 = Some(3u8);
    if let Some(max) = config_max_2 {
        println!("The maximum is configured to be {max}");
    }

    // Demonstrates else use case in if let syntax:

    // Using match expression to count all non-quarter coins we see while also announcing the state of the quarters in the Coin enum.

    let coin = Coin_WithState::Quarter(UsState::Alabama);

    let mut count = 0;
    match &coin {
        // match takes ownership of the value it is matching on, so we use a reference to the coin variable to avoid taking ownership of it and still be able to use it after the match expression.
        Coin_WithState::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }

    // Using if let syntax to do the same thing as the match expression above, but in a more concise way.
    let mut count_2 = 0;
    if let Coin_WithState::Quarter(state) = &coin {
        // if let takes ownership of the value it is matching on, so we use a reference to the coin variable to avoid taking ownership of it and still be able to use it after the if let expression.
        println!("State quarter from {:?}!", state);
    } else {
        count_2 += 1;
    }
}

// Define a function that takes any IpAddrKind as a parameter.
// Added an underscore `_ip_kind` to explicitly tell the compiler we are ignoring this variable for now.
fn route(_ip_kind: IpAddrKind) {}

// Function that takes an Option<132> as a parameter and adds 1 to the value inside the Option if it is Some, otherwise returns None. We are usig <Option<T> match pattern.
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

// Codes that will be executed in the match arms of the dice_roll match expression.
fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}
fn reroll() {}
