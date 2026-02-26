use hello_world::{ authenticate, auth::models::User };

fn main() {
    let user_1: User = User::create(String::from("shaon"), String::from("123456"));
    authenticate(user_1);
    println!("Hello world");
}
