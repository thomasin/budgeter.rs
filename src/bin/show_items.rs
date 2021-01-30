extern crate budgeter;
extern crate diesel;

use self::budgeter::*;
use self::models::*;
use self::diesel::prelude::*;

fn main() {
    use budgeter::schema::items::dsl::*;

    let connection = establish_connection();
    let results = items.limit(5)
        .load::<Item>(&connection)
        .expect("error loading items");

    println!("displaying {} items", results.len());
    for item in results {
        println!("{}", item.name);
    }
}
