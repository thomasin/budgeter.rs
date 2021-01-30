use clap::App;

mod item;

fn main() { 
    let matches = App::new("budgeter")
        .version("1.0")
        .about("2 minute noodles")
        .author("thomasin")
        .subcommand(
            App::new("item")
                .about("manage budget items")
                .subcommand(
                    App::new("list")
                        .about("list budget items")
                )
                .subcommand(
                    App::new("add")
                        .about("add new budget item")
                )
                .subcommand(
                    App::new("edit")
                        .about("add new budget item")
                )
                .subcommand(
                    App::new("delete")
                        .about("add new budget item")
                )
        )
        .get_matches();

    match matches.subcommand() {
        Some(("item", item_matches)) => {
            match item_matches.subcommand() {
                Some(("list", _item_list_matches)) => {
                    item::list_items();
                }
                Some(("add", _item_add_matches)) => {
                    item::add_item();
                }
                Some(("delete", _item_delete_matches)) => {
                    item::delete_item();
                }
                Some(("edit", _item_edit_matches)) => {
                    item::edit_item();
                }
                _ => println!("invalid subcommand"),
            }
        }
        None => println!("no subcommand was used"), // If no subcommand was used it'll match the tuple ("", None)
        _ => println!("invalid subcommand"), // If all subcommands are defined above, anything else is unreachabe!()
    }
}
