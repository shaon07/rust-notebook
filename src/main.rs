fn main() {
    println!("This is simple Rus code for learning if else and loops.");

    // Example of if-else statement
    let number: i32 = 7;

    if number < 10 {
        println!("The number is less than 10.");
    } else {
        println!("The number is 10 or greater.");
    }

    // Example of multiple conditions
    if number % 2 == 0 {
        println!("The number is even.");
    } else if number % 2 != 0 {
        println!("The number is odd.");
    } else {
        println!("This case will never happen.");
    }

    // example of inline if-else (ternary operator style)
    let even_or_odd: &str = if number % 2 == 0 { "Even" } else { "Odd" };
    // must remember that the if-else expression must return the same type in both branches
    // WRONG WAY: let even_or_odd: &str = if number % 2 == 0 {"Even"} else {123}; // this will cause a compile-time error
    println!("The {number} is {even_or_odd} value.");

    // Example of a loop
    let mut count: i32 = 0;
    while count < 5 {
        println!("Count value: {count}");
        count += 1;
    }

    // Example of a loop
    let mut counter: i32 = 0;
    loop {
        counter += 1;
        println!("The counter value is: {counter}");
        if counter >= 10 {
            break;
        }
    }

    // Example of a loop with return value
    let mut loop_counter: i32 = 0;
    let result: i32 = loop {
        loop_counter += 1;
        if loop_counter == 10 {
            break loop_counter * 2;
        }
    };
    println!("The result from the loop is: {result}");

    // Example of a labeled loop
    let mut outer_count: i32 = 0;
    'outer_loop: loop {
        println!("Outer loop count: {outer_count}");
        let mut inner_count: i32 = 0;
        loop {
            println!("  Inner loop count: {inner_count}");
            if inner_count >= 5 {
                break;
            }
            if outer_count >= 5 {
                break 'outer_loop;
            }
            inner_count += 1;
        }
        outer_count += 1;
    }

    // Example of a for loop
    let array: [i32; 5] = [10, 20, 30, 40, 50];
    for item in array {
        println!("Value is {item}");
    }

    // Example of a for loop with a range
    for number in 1..6 {
        println!("Number in range: {number}");
    }

    // Example of a for loop with a range including the end
    for number in 1..=5 {
        println!("Number in inclusive range: {number}");
    }

    // Example of range with utils functions from standard library
    for number in (1..=10).rev() {
        println!("Reversed number in range: {number}");
    }
}
