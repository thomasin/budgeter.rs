extern crate budgeter;
extern crate diesel;

use std::env::args;
use std::io::stdin;
use diesel::sqlite::SqliteConnection;
use diesel::prelude::*;

use budgeter::{establish_connection, create_item};
use budgeter::models::{Item};
use super::cli;

pub fn list_items() {
    use budgeter::schema::items::dsl::*;

    let connection = establish_connection();
    let results = items.limit(5)
        .load::<Item>(&connection)
        .expect("error loading items");

    cli::success(&format!("displaying {} items", results.len()));
    for item in results {
        cli::success(&format!("{}", item.name));
    }
}

pub fn add_item() {
    let connection = establish_connection();

    match try_write_item(&connection) {
        Ok(val) => cli::success(&format!("saved {} items", val)),
        Err(err) => cli::problem(&format!("error saving item: {}", err.to_string())),
    }
}

fn try_write_item(conn: &SqliteConnection) -> std::result::Result<usize, Box<dyn std::error::Error>> {
    cli::message("what would you like your name to be?");
    let mut name = String::new();
    stdin().read_line(&mut name).unwrap();
    let name = &name[..(name.len() - 1)]; // Drop the newline character

    cli::message("how much is this item?");
    let mut amount_str = String::new();
    stdin().read_line(&mut amount_str).unwrap();
    let amount_str = &amount_str[..(amount_str.len() - 1)]; // Drop the newline character
    let amount_f = amount_str.parse::<f32>()?;

    cli::message("how many days does this amount cover?");
    let mut days_str = String::new();
    stdin().read_line(&mut days_str).unwrap();
    let days_str = &days_str[..(days_str.len() - 1)]; // Drop the newline character
    let days_i = days_str.parse::<i32>()?;

    let n_created = create_item(conn, name, &days_i, &amount_f)?;

    Ok(n_created)
}

pub fn edit_item() {
    let id = args().nth(1).expect("edit_item_amount requires a item id")
        .parse::<i32>().expect("invalid id");
    let connection = establish_connection();

    cli::message("enter new amount");
    let mut amount_str = String::new();
    stdin().read_line(&mut amount_str).unwrap();
    let amount_str = &amount_str[..(amount_str.len() - 1)]; // Drop the newline character

    match amount_str.parse::<f32>() {
        Ok(amount_f) => update_amount(&connection, id, amount_f),
        Err(_) => cli::problem("invalid amount pls do a floating point"),
    }
}

fn update_amount(conn: &SqliteConnection, id: i32, amount_f: f32) {
    use budgeter::schema::items::dsl::{items, amount};

    let result = diesel::update(items.find(id))
        .set(amount.eq(amount_f))
        .execute(conn);

    match result {
        Ok(_) => cli::success(&format!("updated item {} amount to {}", id, amount_f)),
        Err(_) => cli::problem("error updating item")
    }        
}

pub fn delete_item() {
    use budgeter::schema::items::dsl::*;

    let target = args().nth(1).expect("expected a target name to match against");
    let pattern = format!("%{}%", target);

    let connection = establish_connection();
    let result = diesel::delete(items.filter(name.like(pattern)))
        .execute(&connection);

    match result {
        Ok(num_deleted) => cli::success(&format!("deleted {} items", num_deleted)),
        Err(_) => cli::problem("error deleting items"),
    }
}