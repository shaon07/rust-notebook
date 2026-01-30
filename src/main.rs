use std::io;

fn main() {
    let array: [i32; 5] = [10, 20, 30, 40, 50];
    let mut access_index = String::new();

    println!("Enter an index to access (0-4):");
    io::stdin()
        .read_line(&mut access_index)
        .expect("there was a problem while taking the buffter input");

    let access_index: i32 = match access_index.trim().parse() {
        Ok(num) => num,
        Err(e) => {
            eprintln!("Invalid input: {}", e);
            return;
        }
    };

    if access_index < 0 || access_index >= (array.len() as i32) {
        eprintln!("Index out of bounds. Please enter a valid index between 0 and 4.");
    } else {
        println!("Element at index {}: {}", access_index, array[access_index as usize]);
    }
}
