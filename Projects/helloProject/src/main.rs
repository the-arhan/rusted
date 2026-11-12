// Functions
// Entry Point
// any function or variable should be written in snake case
// snake case - hello_world
// kebab case - hello-world

fn main() {
    hello_world();
    tell_height(174);
    human_id("Arhan Ali", 20, 174.7);
    let _x  = {
        let price = 5;
        let qty = 10;
        price * qty
    };
    println!("Result is: {}", _x);
    
    // add(5, 6);
    let y = add(4, 5);
    println!("value of Y is: {}", y);
    println!("Value from 'add' is {}.", add(4,6));

    // Calling the BMI function.
    let weight = 70.0;
    let heigth = 1.82;
    let bmi = calculate_bmi(weight, heigth);
    println!("Your BMI is {:.2}.", bmi);
}

// Hoisting - can call a func anywhere in your code.
fn hello_world() {
    println!("Hello, Rust")
}

// you can insert input values
fn tell_height(height: u32) {
    println!("My height is {} cm.", height)
}

// You can insert more than one value
fn human_id(name: &str, age :u32, height: f32) {
    println!("My name is {}, I am {} years old, and my height is {} cm.", name, age, height);

}

// Functions return values
fn add(a: i32, b: i32) -> i32{
    a + b
}

// Expressions and Statements
// Expression: Anything that returns a value.
// Statement: Anything that does not return a value.
// Almost all statements in rust end with a semicolon ';' .
// 1. Variable declarations: let x = 5;
// 2. Function definitions: fn foo() {}
// 3. Control flow statements: if condition { /* code */}, etc.

// Expression Examples -
//      5
//      true | false
//      add(3,4)
//      if condition {value1} else {value2}
//      ({code})


// Final Example

// BMI = weight(kg)/height(m)*height(m)

fn calculate_bmi(weight_kg: f64, height_m: f64) -> f64 {
    weight_kg / (height_m * height_m)
}

