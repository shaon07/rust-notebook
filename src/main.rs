fn main() {
    // by default, variables are immutable, so the following line would cause a compile-time error
    // let a = 50;
    // println!("The value of a is: {}", a);

    // x = 60;
    // println!("The value of x is: {}", x);

    // To make a variable mutable, we use the 'mut' keyword
    let mut x: i32 = 40;
    println!("The value of x is: {}", x);
    x = 60;
    println!("The value of x is: {}", x);

    // now we will learn about constants
    // constants are always immutable and must have a type annotation. mean we need to tell the type while declaring a constant.
    // const PI = 3.14;
    // println!("The value of PI is: {}", PI);

    // The correct way to declare a constant
    const PI: f64 = 3.1416;
    println!("The value of PI is: {}", PI);

    // we can not use the 'mut' keyword with constants because they are always immutable. and can not change their value in entire program.
    const MAX_POINTS: u32 = 100_000;
    println!("The maximum points are: {}", MAX_POINTS);

    // if we do like this, it will cause a compile-time error
    // const mut MAX_POINTS = 200_000; or const mut MAX_POINTS: u32 = 200_000;

    // const can be declared in any scope, including the global scope, whereas mutable variables can only be declared within functions or blocks. so constants have a broader scope of accessibility.
    const GLOBAL_CONSTANT: i32 = 10;
    println!("The value of GLOBAL_CONSTANT is: {}", GLOBAL_CONSTANT);

    // we can not use dynamic values into a constant. for example, the result of a function call or a value that is determined at runtime or may change cannot be assigned to a constant.
    // let five = 5;
    // const FIVE_TIMES_TWO: i32 = five * 2;
    // println!("The value of FIVE_TIMES_TWO is: {}", FIVE_TIMES_TWO);

    // so the correct way is:
    const FIVE_TIMES_TWO: i32 = 5 * 2;
    println!("The value of FIVE_TIMES_TWO is: {}", FIVE_TIMES_TWO);
}
