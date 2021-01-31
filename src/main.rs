use clap::{App, Arg};
use budgeter::*;

mod budget;
mod item;
mod interact;
mod cli;

fn main() {
    env_logger::init();

    let matches = App::new("budgeter")
        .version("1.0")
        .about("2 minute noodles")
        .author("thomasin")
        .subcommand(App::new("item")
            .about("manage budget items")
            .subcommand(App::new("list")
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
        .subcommand(App::new("create")
            .about("create a budget for time period")
            .arg(Arg::new("duration")
                .about("number of days to run budget for")
                .required(true)
                .index(1)))
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
        Some(("create", create_matches)) => {
            match create_matches.value_of_t::<i32>("duration") {
                Ok(_duration) => Err(Box::from("invalid duration given")),
                Err(_) => Err(Box::from("invalid duration given")),
            }
        }
        None => Err(Box::from("no subcommand was used")),
        _ => Err(Box::from("invalid subcommand")),
    };

    match result {
        Ok(msg) => cli::success(&msg),
        Err(err) => cli::problem(&err.to_string()),
    }
}

