fn main() {
    let x: i32 = 5;

    println!("The value of x is: {x}");

    hello();

    let sum: i32 = add(10, 20);
    println!("The sum of 10 and 20 is: {sum}");

    // Nested function example
    fn nested_function() {
        println!("This is a nested function!");
    }

    nested_function();
}

// Global functions
// this is a regular function
fn hello() {
    println!("Hello, world!");
}

// this is a function with parameters and a explicit return value
fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

// alternatively, you can omit the 'return' keyword for the last expression. BUT BE CAREFUL WITH THE SEMICOLON! (if you put a semicolon, it becomes a statement and returns nothing)
// Right way:
// fn add(a: i32, b: i32) -> i32 {
//     a + b
// }

// Wrong way:
// fun add(a: i32, b:i32) -> i32 {
// a+ b; // this will cause a compile-time error because the function returns nothing
// }
