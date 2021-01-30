extern crate budgeter;
extern crate diesel;

use std::io::stdin;
use diesel::sqlite::SqliteConnection;

use self::diesel::prelude::*;
use self::budgeter::*;

use std::env::args;

fn main() {
    let id = args().nth(1).expect("edit_item_amount requires a item id")
        .parse::<i32>().expect("invalid id");
    let connection = establish_connection();

    println!("enter new amount");
    let mut amount_str = String::new();
    stdin().read_line(&mut amount_str).unwrap();
    let amount_str = &amount_str[..(amount_str.len() - 1)]; // Drop the newline character

    match amount_str.parse::<f32>() {
        Ok(amount_f) => update_amount(&connection, id, amount_f),
        Err(_) => println!("invalid amount pls do a floating point"),
    }
}

fn update_amount(conn: &SqliteConnection, id: i32, amount_f: f32) {
    use budgeter::schema::items::dsl::{items, amount};

    let result = diesel::update(items.find(id))
        .set(amount.eq(amount_f))
        .execute(conn);

    match result {
        Ok(_) => println!("updated item {} amount to {}", id, amount_f),
        Err(_) => println!("error updating item")
    }        
}
