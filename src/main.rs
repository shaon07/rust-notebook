fn main() {
    // in rust slices a part of strings that refer back us the value;
    let full_name = String::from("Shaon ali khan");
    let first_name = &full_name[0..5]; // [start_point...exclusive_end_point]. its not mandatory to pass start_point 0 if we start counting from 0. so that we can also use [..6] same as [0..6];

    let last_name = &full_name[10..];

    println!("{first_name} {last_name}");
}
