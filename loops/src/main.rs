// A SEPARATE function to demonstrate `return`
fn calculate_with_return() -> i32 {
    let mut counter = 0;
    loop {
        counter += 1;
        if counter == 10 {
            // Exits the ENTIRE function and hands back an i32
            return counter * 2; 
        }
    }
}
fn main() {
// CONTINUOUS LOOP (MUST INTERRUPT WITH CTRL C)
// loop {
// println!("again!");
// }

let mut counter = 0;
// THE RUST RETURN RULE:
// 1. Implicit return: The value "falls out" the bottom. (NO SEMICOLON)
//      Example: x * 2
// 2. Explicit action: You force it out with `break` or `return`. (SEMICOLON REQUIRED)
//      Example: return x * 2; OR break x * 2;
let result = loop {
counter += 1;
if counter == 10 {
break counter * 2;
}

};
println!("The result is {result}");

// Calling the function that uses `return`
    let return_result = calculate_with_return();
    println!("The result from the return function is: {return_result}");

        let mut count = 0;
        'counting_up: loop { // outer loop has label "counting_up" that helps break it usingthe inner loop
            println!("count = {count}");
            let mut remaining = 10;
                loop {
                    println!("remaining = {remaining}");
                    if remaining == 9 {
                        break;
                    }
                    if count == 2 {
                        break 'counting_up;
                    }
                    remaining -= 1;
                }
            count += 1;
        }
        println!("End count = {count}");

        let mut number = 3;
        while number != 0 {
        println!("{number}!");
        number -= 1;
        }
        println!("LIFTOFF!!!");

        // using while loop to print array loop (
        // slow because compiler must check if index in within bounds in every iteration,
        // and error prone because it might cause array to panic if size of array size is decreased
        // or miss items if array size is increased)
        let a = [10, 20, 30, 40, 50];
        let mut index = 0;
        while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
        }

        // for loop a more concise way
        let a = [10, 20, 30, 40, 50];
        for element in a {
        println!("the value is: {element}");
        }

        // doesn't include 4
        for number in (1..4).rev() {
        println!("{number}!");
        }
        println!("LIFTOFF!!!");

        // ASSIGNMENT:
        // Convert temperatures between Fahrenheit and Celsius.
        // Generate the nth Fibonacci number.
        // Print the lyrics to the Christmas carol “The Twelve Days of Christmas,”
        // taking advantage of the repetition in the song.

}

