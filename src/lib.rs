#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

use dotenv::dotenv;
use std::env;

use self::models::{Item, NewItem};

pub mod schema;
pub mod models;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn create_item<'a>(conn: SqliteConnection, name: &'a str, days: &'a i32, amount: &'a f32) -> QueryResult<usize> {
    use schema::items;

    let new_item = NewItem {
        name: name,
        days: days,
        amount: amount,
    };

    diesel::insert_into(items::table)
        .values(&new_item)
        .execute(&conn)
}

pub fn list_items(conn: SqliteConnection) -> std::result::Result<Vec<Item>, diesel::result::Error> {
    use schema::items::dsl::{items};

    items.limit(5)
        .load::<Item>(&conn)
}

pub fn edit_item(conn: SqliteConnection, id: i32, new_name: &str, new_days: &i32, new_amount: &f32) -> std::result::Result<usize, diesel::result::Error> {
    use schema::items::dsl::{items, name, days, amount};

    diesel::update(items.find(id))
        .set((name.eq(new_name), days.eq(new_days), amount.eq(new_amount)))
        .execute(&conn)
}

pub fn delete_item(conn: SqliteConnection, id: i32) -> std::result::Result<usize, diesel::result::Error> {
    use schema::items::dsl::{items};

    diesel::delete(items.find(id))
        .execute(&conn)
}

