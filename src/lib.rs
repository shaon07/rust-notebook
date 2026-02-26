pub struct User {
    name: String,
    email: String,
    password: String,
    age: u8,
}

impl User {
    pub fn create_user(name: String, email: String, password: String, age: u8) -> Self {
        Self { name, email, password, age }
    }
}

pub fn show_user_details(user: User) {
    println!("Name: {}", user.name);
    println!("Email: {}", user.email);
    println!("Password: {}", user.password);
    println!("Age: {}", user.age)
}
