extern crate budgeter;
extern crate diesel;

use std::io::stdin;
use diesel::sqlite::SqliteConnection;

use self::budgeter::*;

fn main() {
    let connection = establish_connection();

    match try_write_item(&connection) {
        Ok(val) => println!("saved {} items", val),
        Err(_) => println!("error saving item"),
    }
}

fn try_write_item(conn: &SqliteConnection) -> std::result::Result<usize, Box<dyn std::error::Error>> {
    println!("what would you like your name to be?");
    let mut name = String::new();
    stdin().read_line(&mut name).unwrap();
    let name = &name[..(name.len() - 1)]; // Drop the newline character

    println!("how much is this item?");
    let mut amount_str = String::new();
    stdin().read_line(&mut amount_str).unwrap();
    let amount_str = &amount_str[..(amount_str.len() - 1)]; // Drop the newline character
    let amount_f = amount_str.parse::<f32>()?;

    println!("how many days does this amount cover?");
    let mut days_str = String::new();
    stdin().read_line(&mut days_str).unwrap();
    let days_str = &days_str[..(days_str.len() - 1)]; // Drop the newline character
    let days_i = days_str.parse::<i32>()?;

    let n_created = create_item(conn, name, &days_i, &amount_f)?;

    Ok(n_created)
}
