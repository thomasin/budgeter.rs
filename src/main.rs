#[macro_use]
extern crate diesel;

use self::libr::establish_connection;
use clap::{App, Arg};

mod interact;
mod cli;
mod duration;
mod budget;
mod models;
mod schema;
mod libr;

fn main() {
    env_logger::init();

    let matches = App::new("budgeter")
        .version("1.0")
        .about("2 minute noodles")
        .author("thomasin")
        .subcommand(App::new("item")
            .about("manage budget items")
            .subcommand(App::new("list")
                .alias("ls")
                .about("list budget items"))
            .subcommand(App::new("add")
                .about("add new budget item"))
            .subcommand(App::new("edit")
                .about("add new budget item")
                .arg(Arg::new("id")
                    .about("item id to edit")
                    .required(true)
                    .index(1)))
            .subcommand(App::new("delete")
                .about("add new budget item")
                .arg(Arg::new("id")
                    .about("item id to edit")
                    .required(true)
                    .index(1))))
        .subcommand(App::new("show")
            .about("show a budget for time period"))
        .get_matches();

    let conn = establish_connection();

    let result = match matches.subcommand() {
        Some(("item", item_matches)) => {
            match item_matches.subcommand() {
                Some(("list", _)) => {
                    interact::list_all_items(conn)
                }

                Some(("add", _)) => {
                    interact::add_item(conn)
                }

                Some(("edit", edit_matches)) => {
                    match edit_matches.value_of_t::<i32>("id") {
                        Ok(id) => {
                            interact::edit_item(conn, id)
                        }
                        Err(_) => Err(Box::from("invalid id given")),
                    }
                }

                Some(("delete", delete_matches)) => {
                    match delete_matches.value_of_t::<i32>("id") {
                        Ok(id) => {
                            interact::remove_item(conn, id)
                        }
                        Err(_) => Err(Box::from("invalid id given")),
                    }
                }

                _ => Err(Box::from("invalid subcommand")),
            }
        }

        Some(("show", _)) => {
            interact::show_budget(conn)
        }

        None => Err(Box::from("no subcommand was used")),

        _ => Err(Box::from("invalid subcommand")),
    };

    match result {
        Ok(_) => (), // any success messages should be shown already
        Err(err) => cli::problem(&err.to_string()),
    }
}

