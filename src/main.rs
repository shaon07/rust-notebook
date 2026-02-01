fn main() {
    let s1 = String::from("Hello");
    println!("{s1}");
    let s2 = s1;
    println!("{s2}");

    // the following s1 will not works bcoz the s1 ownership is moved to s2.
    //println!("{s1}");

    // in rust their are two type of mamory allocation
    // 1. is in Stack -> the fixed size variables. like int, array, tuples,
    // 2. is in Heap -> unknown size. like this this string.
    // so every variables are store in heap need to manage their ownership

    // fixed size example
    let num1: i32 = 10;
    let num2: i32 = num1;

    println!("Num1: {num1}");
    println!("Num2: {num2}");

    let x: i32 = 5;
    let y: i32 = x;

    println!("x = {x}, y = {y}");
    // this works becouse fixed size variables no need ownership.

    // but for String it store in heap. so mamory management need to handler by the rust compilers. so it uses ownership.

    // to avoid the owership we can use clone method to take the values not the ownership. but it expensive but neccesary in certain conditions.

    let s3 = String::from("Hi");
    println!("{s3}");
    let s4 = s3.clone();
    println!("{s4}");

    // now we can update each variables value indepently. like:
    let mut s5 = s4.clone();
    s5.push_str(" how are you");
    println!("{s5}");

    // another way to take ownsership in functions:
    let name = String::from("Bangladesh");
    take_ownsership(name); // now the name ownership in moved to the take_ownsership functions. so we cant use the name after the function call.

    // here we cant use name variables. coz it no longer available here.
    // println!("{name}"); this line of code will throw error.

    let my_custom_name = String::from("Shaon");
    let my_custom_name = take_and_give_ownership(my_custom_name);
    println!("My custom name is: {my_custom_name}");

    let s6 = String::from("Hello, world!");
    let (s7, len) = calculate_length(s6);
    println!("The length of '{}' is {}.", s7, len);
}

fn take_ownsership(name: String) {
    println!("{name}");
}

fn take_and_give_ownership(name: String) -> String {
    let custom_name = name + " from Bangladesh";
    println!("This is custom name: {custom_name}");
    return custom_name;
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    return (s, length);
}
