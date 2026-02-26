pub mod models;
use crate::enums::db::{ DbStatus };
use crate::db::{ connect_db };

pub fn login(user: models::User) {
    if let DbStatus::Connected = connect_db() {
        println!("User logged in with username: {} and password: {}", user.username, user.password)
    } else {
        println!("Failed to connect with Database");
    }
}
