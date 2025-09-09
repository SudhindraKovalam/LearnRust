fn main() {
    variables();
    arithmetic_operations(10, 5);
    type_inference();
}

fn variables() {
    println!("Hello, variables!");
    let mut x: i32 = 10; // Initialize x with the value 10 using the let keyword and adding the mut keyword to make it mutable
    println!("The value of x is: {}", x);
    x = 20; // Assign a new value to x
    println!("x: {x}");
}

fn arithmetic_operations(a: i32, b: i32) {
    println!("Addition: {}", a + b);
    println!("Subtraction: {}", a - b);
    println!("Multiplication: {}", a * b);
    println!("Division: {}", a / b);
    println!("Remainder: {}", a % b);
}

fn type_inference() {
    let x = 10;
    let y = 20;

    takes_u32(x);
    takes_i8(y);
    // takes_u32(y);
}

fn takes_u32(x: u32) {
    println!("u32: {x}");
}

fn takes_i8(y: i8) {
    println!("i8: {y}");
}
