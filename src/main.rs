fn main() {
    let mut items: Vec<i32> = vec![1, 2, 3, 4];
    // 1. We create a mutable vector with 4 elements.
    //    Index:   0  1  2  3
    //    Values: [1, 2, 3, 4]

    let val = items[3];
    // 2. items[3] accesses the 4th element (value 4).
    //    Because i32 implements Copy, Rust **copies** the value into `val`.
    //    So now: val = 4
    //    The vector is still [1, 2, 3, 4]

    items[3] = 8;
    // 3. We change the element at index 3 to 8.
    //    The vector is now [1, 2, 3, 8]

    println!("{} and {}", items.get(3).unwrap_or(&4), val);
    // 4. This line prints two things:
    //    a) items.get(3) → returns Option<&i32> = Some(&8)
    //       .unwrap_or(&4) → because we have Some, it gives us &8
    //       So it prints 8
    //
    //    b) val → still contains the old copy (4)
    //
    //    Output: 8 and 4
}
