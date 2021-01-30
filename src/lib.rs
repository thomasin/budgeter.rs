#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;

use self::models::{NewItem};

pub mod schema;
pub mod models;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn create_item<'a>(conn: &SqliteConnection, name: &'a str, days: &'a i32, amount: &'a f32) -> QueryResult<usize> {
    use schema::items;

    let new_item = NewItem {
        name: name,
        days: days,
        amount: amount,
    };

    diesel::insert_into(items::table)
        .values(&new_item)
        .execute(conn)
}