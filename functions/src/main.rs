fn main() {
    println!("Hello, world!");
    another_function();
    f1(5);
    print_labeled_measurement(5, 'h');

    // expressions evaluate to a value, and can be part of a statement. Statements do not return a value.
    let y = {
    let x = 3;
    x + 1 //line without a semicolon at the end
    };
    // Expressions do not include ending semicolons. If you add a semicolon to the
    // end of an expression, you turn it into a statement, and it will then not return a
    // value.
    println!("The value of y is: {y}");

    let x = five();
    println!("The value of x is: {x}");

    let x = plus_one(5);
    println!("The value of x is: {x}");



}
fn another_function() {
println!("Another function.");
}
fn f1(x: i32) {
    println!("The value of x is: {x}");
}
fn print_labeled_measurement(value: i32, unit_label: char) {
println!("The measurement is: {value}{unit_label}");
}
fn five() -> i32 {
5
}
fn plus_one(x: i32) -> i32 {
x + 1
}