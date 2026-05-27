fn main() {
// Mutable variables can be changed after they are declared. To make a variable mutable, we use the mut keyword before the variable name.
let mut x = 5;
println!("The value of x is: {x}");
x = 6;
println!("The value of x is: {x}");

// Shadowing allows us to declare a new variable with the same name as a previous variable. The new variable shadows the previous variable, and we can use the same name for different variables in different scopes.
// difference between mutability and shadowing is the keywords mut and let. mut allows us to change the value of a variable, while let allows us to declare a new variable with the same name as a previous variable.
let y = 5;
let y = y + 1;
let y = y * 2;
println!("The value of y is: {y}");

let x = 2.0; // f64
let y: f32 = 3.0; // f32
println!("The value of x is: {x}");
println!("The value of y is: {y}");

// addition
let sum = 5 + 10;
// subtraction
let difference = 95.5 - 4.3;
// multiplication
let product = 4 * 30;
// division
let quotient = 56.7 / 32.2;
let truncated = -5 / 3; // results in -1
// remainder
let remainder = 43 % 5;
println!("The sum is: {sum}");
println!("The difference is: {difference}");
println!("The product is: {product}");
println!("The quotient is: {quotient}");
println!("The truncated is: {truncated}");
println!("The remainder is: {remainder}");

let t = true;
let f: bool = false; // with explicit type annotation
println!("The value of t is: {t}");
println!("The value of f is: {f}");

 {
let c = 'z';
let z: char = 'ℤ'; // with explicit type annotation
let heart_eyed_cat = '😻';
println!("The value of c is: {c}");
println!("The value of z is: {z}");
println!("The value of heart_eyed_cat is: {heart_eyed_cat}");
}

// tuples
let tup: (i32, f64, u8) = (500, 6.4, 1);
// destructuring a tuple
let (x, y, z) = tup;
println!("The value of x is: {x}");
println!("The value of y is: {y}");
println!("The value of z is: {z}");

let five_hundred = tup.0;
let six_point_four = tup.1;
let one = tup.2;
println!("The value of five_hundred is: {five_hundred}");
println!("The value of six_point_four is: {six_point_four}");
println!("The value of one is: {one}");

// arrays
let a: [i32; 5] = [1, 2, 3, 4, 5];
println!("The value of a is: {a:?}");
// pretty print an array
println!("The value of a is: {:#?}", a);
// OR
println!("The value of a is: {a:#?}");
let first = a[0];
let second = a[1];
println!("The value of first is: {first}");
println!("The value of second is: {second}");


let b = [3; 5];
println!("The value of b is: {b:#?}");



}
