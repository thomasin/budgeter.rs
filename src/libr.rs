extern crate dotenv;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;

use crate::models::{Item, NewItem};
use crate::duration::{Duration};
use crate::budget::{Budget};


pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn create_item(conn: SqliteConnection, name: String, duration: Duration, cost: f32, currency: String) -> QueryResult<usize> {
    use crate::schema::items;

    let new_item = NewItem {
        name,
        duration,
        cost,
        currency,
    };

    diesel::insert_into(items::table)
        .values(new_item)
        .execute(&conn)
}

pub fn list_items(conn: SqliteConnection) -> std::result::Result<Vec<Item>, diesel::result::Error> {
    use crate::schema::items::dsl::{items};

    items.load::<Item>(&conn)
}

pub fn edit_item(conn: SqliteConnection, id: i32, new_name: String, new_duration: Duration, new_cost: f32, new_currency: String) -> std::result::Result<usize, diesel::result::Error> {
    use crate::schema::items::dsl::{items, name, duration_unit, duration_amount, cost, currency};

    diesel::update(items.find(id))
        .set((
            name.eq(new_name),
            duration_unit.eq(new_duration.unit()),
            duration_amount.eq(new_duration.amount()),
            cost.eq(new_cost),
            currency.eq(new_currency)
        ))
        .execute(&conn)
}

pub fn delete_item(conn: SqliteConnection, id: i32) -> std::result::Result<usize, diesel::result::Error> {
    use crate::schema::items::dsl::{items};

    diesel::delete(items.find(id))
        .execute(&conn)
}

pub fn show_budget(conn: SqliteConnection, duration: Duration) -> std::result::Result<Budget, diesel::result::Error> {
    let items = list_items(conn)?;

    let budget = Budget { duration, items };

    Ok(budget)
}
