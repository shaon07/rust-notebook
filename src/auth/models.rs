pub struct User {
    pub username: String,
    pub password: String,
}

impl User {
    pub fn create(username: String, password: String) -> Self {
        Self { username, password }
    }
}
