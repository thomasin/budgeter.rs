extern crate budgeter;
extern crate diesel;

use diesel::prelude::*;

use budgeter::{establish_connection};
use budgeter::models::{Item};
use super::cli;

pub fn create_for(duration: i32) {
    use budgeter::schema::items::dsl::*;

    let connection = establish_connection();
    let results = items.load::<Item>(&connection)
        .expect("error loading items");

    cli::success(&format!("loaded {} items", results.len()));

    let sum = results.iter().fold(0.0, |acc, item| {
        let multiple = item.days as f32 / duration as f32;
        acc + (item.amount * multiple)
    });

    cli::success(&format!("budget is {}", sum))
}
