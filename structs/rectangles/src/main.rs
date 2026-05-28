#[derive(Debug)] // We can use the #[derive(Debug)] attribute to automatically generate an implementation of the Debug trait for our struct, which allows us to print the struct using the {:?} format specifier in a println! macro.
struct Rectangle {
width: u32,
height: u32,
}

struct Neuralnetwork {
    weight: u32,
    bias: u32,
}

// Using struct to implement methods is a common practice in Rust. We can define methods on a struct using an impl block. Here's how we can define a method to calculate the area of a rectangle:
impl Rectangle {
fn method_area(&self) -> u32 {
self.width * self.height
}
fn width(&self) -> bool { // This method checks if the width of the rectangle is greater than 0 and returns a boolean value indicating whether the rectangle has a non-zero width.
self.width > 0
}

// We can also define an associated function that is not a method, meaning it does not take self as a parameter. Associated functions are often used for constructors that return a new instance of the struct. We return capitalized Self from the function to return an instance of the struct that the impl block is for.
fn square(size: u32) -> Self { 
        Self {
            width: size,
            height: size,
        }
    }

}

struct Point { x: i32, y: i32 }

impl Point {
    // This method wants a REFERENCE to a Point (&self)
    fn print_point(&self) { 
        println!("{}, {}", self.x, self.y);
    }
}

impl Rectangle {
fn area(&self) -> u32 {
self.width * self.height
}
// Methods can take multiple parameters that we add to the signature after the self parameter, and those parameters work just like parameters in functions.
fn can_hold(&self, other: &Rectangle) -> bool {
self.width > other.width && self.height > other.height
}
}


// Using multiple impl blocks for organization:
// Block 1: Constructors and initialization
impl Neuralnetwork {
    fn new(weight: u32, bias: u32) -> Self { 
        // Return a new instance of Self
        Self { weight, bias }
     }
    fn transformer(size: u32) -> Self {
        // Return a new instance using 'size' for both fields
        Self { weight: size, bias: size }
     }
}

// Block 2: Math and calculations
impl Neuralnetwork {
    fn forward_prop(&self) -> u32 {
        // Correctly returning a u32 math calculation
        self.weight * self.bias
      }
    fn back_prop(&self) -> u32 {
        // Returning a dummy u32 (0) just to satisfy the compiler for now
        0
     }
}

// Using multiple impl blocks for implementing traits:

// A completely separate block just for the 'Default' trait
impl Default for Neuralnetwork {
    fn default() -> Self {
        Neuralnetwork { weight: 0, bias: 0 }
}
}

// This program calculates the area of a rectangle using a function that takes the width and height as parameters and returns the area. The area is calculated by multiplying the width and height together, and the result is printed to the console.
fn main() {
let width1 = 30;
let height1 = 50;
println!(
"The area of the rectangle is {} square pixels.",
area(width1, height1)
);

// We can also use a tuple to hold the dimensions of the rectangle and pass that tuple to a function. Here's how we can do that:
let rect1 = (30, 50);
println!(
    "The area of the rectangle is {} square pixels.",
 tuple_area(rect1)
);

let rect1 = Rectangle {
width: 30,
height: 50,
};

println!(
"The area of the rectangle is {} square pixels.",
struct_area(&rect1)
);

println!("rect1 is {rect1:?}"); // This will print the debug representation of rect1, which includes the values of its fields. The output will look something like this: rect1 is Rectangle { width: 30, height: 50 }

println!("rect1 is {rect1:#?}"); // This will print the debug representation of rect1 in a more human-readable format, with each field on a new line.

let scale = 2;
let rect1 = Rectangle {
 width: dbg!(30 * scale), // The dbg! macro will print the value of the expression (30 * scale) to the console, along with the file and line number where the dbg! macro is called. This can be useful for debugging purposes, as it allows you to see the intermediate values of your calculations.
height: 50,
};
 dbg!(&rect1); // The dbg! macro can also be used to print the value of an entire struct.
             // We use &rect instead of rect1 because dbg! retuurns the value it is given, and there is no value to return if we give it rect1.


// Calling the method_area method on the rect1 instance of the Rectangle struct to calculate and print the area of the rectangle. The method_area method is defined in the impl block for the Rectangle struct, and it takes a reference to self (the instance of the struct) and returns the area by multiplying the width and height fields of the struct together.
let rect1 = Rectangle {
width: 30,
height: 50,
};
println!(
"The area of the rectangle is {} square pixels.",
rect1.method_area()
);

let rect1 = Rectangle {
width: 30,
height: 50,
};
if rect1.width() {
println!(
"The rectangle has a nonzero width; it is {}",
rect1.width
);
}

let p = Point { x: 5, y: 10 }; // p is an actual Point object

// it automatically references p by adding & infront of p to call the print_point method, which expects a reference to a Point (&self). This is a convenient feature of Rust called "automatic referencing.
 // You just type this:
p.print_point(); 

// The compiler quietly rewrites it to this before compiling:
(&p).print_point();


let p = Point { x: 5, y: 10 };
let ref_p = &p;        // A reference
let double_ref_p = &&p; // A reference to a reference!

// it automatically dereferences double_ref_p by adding * in front of it to call the print_point method, which expects a reference to a Point (&self). This is another convenient feature of Rust called "automatic dereferencing."
double_ref_p.print_point();

// The compiler strips away the extra references by rewriting it to:
(*double_ref_p).print_point();

let rect2 = Rectangle {
width: 10,
height: 40,
};

let rect3 = Rectangle {
width: 60,
height: 45,
};
println!("Can rect1 hold rect2? {}",
rect1.can_hold(&rect2));
println!("Can rect1 hold rect3? {}",
rect1.can_hold(&rect3));



// We ask the Rectangle "namespace" to run the square function.
// This creates a new Rectangle with width 3 and height 3.
// it uses :: instead of . because square is an associated function, not a method.
let sq = Rectangle::square(3); 
    
// We already know another famous associated function:
let s = String::from("hello");




}
fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn tuple_area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn struct_area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
