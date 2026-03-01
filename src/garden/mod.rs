use crate::garden::vegetables::Asparagus;

pub mod vegetables;

pub fn hello() {
    let user: Asparagus = Asparagus {
        name: String::from("Hello world"),
    };

    println!("{}", user.name);
}
