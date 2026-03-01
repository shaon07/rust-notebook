use hello_world::{ garden, hotel::front_of_the_house::hosting::seat_at_table };

fn main() {
    println!("Hello world");
    garden::hello();
    seat_at_table();
}
