use clap::{App, Arg};

mod budget;
mod item;
mod cli;

fn main() { 
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
                .about("add new budget item"))
            .subcommand(App::new("delete")
                .about("add new budget item")))
        .subcommand(App::new("create")
            .about("create a budget for time period")
            .arg(Arg::new("duration")
                .about("number of days to run budget for")
                .required(true)
                .index(1)))
        .get_matches();

    match matches.subcommand() {
        Some(("item", item_matches)) => {
            match item_matches.subcommand() {
                Some(("list", _)) => {
                    item::list_items();
                }
                Some(("add", _)) => {
                    item::add_item();
                }
                Some(("delete", _)) => {
                    item::delete_item();
                }
                Some(("edit", _)) => {
                    item::edit_item();
                }
                _ => cli::problem("invalid subcommand"),
            }
        }
        Some(("create", create_matches)) => {
            match create_matches.value_of_t::<i32>("duration") {
                Ok(duration) => budget::create_for(duration),
                Err(_) => cli::problem("invalid duration given"),
            }
        }
        None => cli::problem("no subcommand was used"), // If no subcommand was used it'll match the tuple ("", None)
        _ => cli::problem("invalid subcommand"), // If all subcommands are defined above, anything else is unreachabe!()
    }
}
