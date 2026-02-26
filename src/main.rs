use hello_world::{ User, show_user_details };

fn main() {
    let user_1: User = User::create_user(
        String::from("shaon ali"),
        String::from("hello@gmail.com"),
        String::from("123456"),
        26
    );

    show_user_details(user_1);
    println!("Hello world");
}
