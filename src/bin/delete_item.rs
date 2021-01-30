extern crate budgeter;
extern crate diesel;

use self::diesel::prelude::*;
use self::budgeter::*;
use std::env::args;

fn main() {
    use budgeter::schema::items::dsl::*;

    let target = args().nth(1).expect("expected a target name to match against");
    let pattern = format!("%{}%", target);

    let connection = establish_connection();
    let result = diesel::delete(items.filter(name.like(pattern)))
        .execute(&connection);

    match result {
        Ok(num_deleted) => println!("deleted {} items", num_deleted),
        Err(_) => println!("error deleting items"),
    }
}
