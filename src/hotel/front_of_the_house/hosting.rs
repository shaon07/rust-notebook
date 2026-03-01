// use crate::hotel::front_of_the_house;

fn add_to_waitlist() {
    println!("Add to waitList");
}

pub fn seat_at_table() {
    add_to_waitlist();

    // this is called absolute path
    crate::hotel::front_of_the_house::serving::serve_order();

    // this is related path
    // front_of_the_house::serving::serve_order();
}
