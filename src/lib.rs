pub mod db;
pub mod auth;
mod enums;

pub fn authenticate(user: auth::models::User) {
    auth::login(user);
}
