use std::char;

fn main() {
    // in rust their are mainly two types of datatypes
    // 1. Scalar types-(integers, floating-point numbers, booleans, and characters)
    // 2. Compound types-(tuples and arrays)

    let unsigned_u8_integer: u8 = 255; // 0 to 255
    println!("Unsigned u8 integer: {}", unsigned_u8_integer);
    let signed_i8_integer: i8 = -128; // -128 to 127
    println!("Signed i8 integer: {}", signed_i8_integer);

    let unsigned_u16_integer: u16 = 65_535; // 0 to 65,535
    println!("Unsigned u16 integer: {}", unsigned_u16_integer);
    let signed_i16_integer: i16 = -32_768; // -32,768 to 32,767
    println!("Signed i16 integer: {}", signed_i16_integer);

    let unsigned_u32_integer: u32 = 4_294_967_295; // 0 to 4,294,967,295
    println!("Unsigned u32 integer: {}", unsigned_u32_integer);
    let signed_i32_integer: i32 = -2_147_483_648; // -2,147,483,648 to 2,147,483,647
    println!("Signed i32 integer: {}", signed_i32_integer);

    let unsigned_u64_integer: u64 = 18_446_744_073_709_551_615; // 0 to 18,446,744,073,709,551,615
    println!("Unsigned u64 integer: {}", unsigned_u64_integer);
    let signed_i64_integer: i64 = -9_223_372_036_854_775_808; // -9,223,372,036,854,775,808 to 9,223,372,036,854,775,807
    println!("Signed i64 integer: {}", signed_i64_integer);

    let float_32: f32 = 3.14; // 32-bit floating point
    println!("32-bit floating point: {}", float_32);
    let float_64: f64 = 2.718281828459045; // 64-bit floating point
    println!("64-bit floating point: {}", float_64);

    // Boolean type only has two values: true and false
    let boolen_true_value: bool = true; // Boolean type
    println!("Boolean value: {}", boolen_true_value);
    let boolean_false_value: bool = false; // Boolean type
    println!("Boolean value: {}", boolean_false_value);

    // Character type represents a single Unicode scalar value and is specified with single quotes
    let character_a: char = 'A'; // Character type
    println!("Character: {}", character_a);
    let character_heart: char = '❤'; // Character type
    println!("Character: {}", character_heart);

    // Tuple type can group multiple values of different types into a single compound type
    let tuple_example: (i32, f64, char) = (42, 3.14, 'R'); // Tuple type
    println!("Tuple: ({}, {}, {})", tuple_example.0, tuple_example.1, tuple_example.2);
    let (x, y, z) = tuple_example; // Destructuring the tuple
    println!("Destructured Tuple: ({}, {}, {})", x, y, z);

    // Array type is a collection of values of the same type with a fixed length
    let array_example: [i32; 5] = [1, 2, 3, 4, 5]; // Array type
    println!("Array: {:?}", array_example);
    let first_element = array_example[0]; // Accessing array element
    println!("First element of the array: {}", first_element);

    let array_of_strings: [&str; 3] = ["Hello", "World", "Rust"]; // Array of string slices
    println!("Array of strings: {:?}", array_of_strings);

    // auto-fill array
    let auto_filled_array: [i32; 10] = [0; 10]; // Array of ten zeros
    println!("Auto-filled array: {:?}", auto_filled_array);
}
