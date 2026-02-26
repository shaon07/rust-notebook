#[allow(dead_code, unused)]
use crate::enums::db;
use db::{ DbStatus };

pub fn connect_db() -> DbStatus {
    println!("Connecting to DB...");
    DbStatus::Connected
}
