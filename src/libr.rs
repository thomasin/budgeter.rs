extern crate dotenv;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;

use crate::models::{Item, NewItem};
use crate::duration::{Duration};

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn create_item(conn: SqliteConnection, name: String, duration: Duration, cost: f32) -> QueryResult<usize> {
    use crate::schema::items;

    let new_item = NewItem {
        name,
        duration,
        cost,
    };

    diesel::insert_into(items::table)
        .values(new_item)
        .execute(&conn)
}

pub fn list_items(conn: SqliteConnection) -> std::result::Result<Vec<Item>, diesel::result::Error> {
    use crate::schema::items::dsl::{items};

    items.load::<Item>(&conn)
}

pub fn edit_item(conn: SqliteConnection, id: i32, new_name: &str, new_duration_amount: &i32, new_cost: &f32) -> std::result::Result<usize, diesel::result::Error> {
    use crate::schema::items::dsl::{items, name, duration_amount, cost};

    diesel::update(items.find(id))
        .set((name.eq(new_name), duration_amount.eq(new_duration_amount), cost.eq(new_cost)))
        .execute(&conn)
}

pub fn delete_item(conn: SqliteConnection, id: i32) -> std::result::Result<usize, diesel::result::Error> {
    use crate::schema::items::dsl::{items};

    diesel::delete(items.find(id))
        .execute(&conn)
}

pub fn show_budget(conn: SqliteConnection, duration: i32) -> std::result::Result<f32, diesel::result::Error> {
    let items = list_items(conn)?;

    let sum = items.iter().fold(0.0, |acc, item| {
        // item.duration_amount
        let multiple = duration as f32 / 3_f32;
        acc + (item.cost * multiple)
    });

    Ok(sum)
}

