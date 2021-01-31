use diesel::SqliteConnection;

use std::error;
use log;

use super::cli;

pub fn list_all_items(conn: SqliteConnection) -> std::result::Result<String, Box<dyn error::Error>> {
    let items = budgeter::list_items(conn)?;
    
    let headers = vec!["id".to_string(), "name".to_string(), "amount".to_string(), "days".to_string()];
    let mut table = cli::Table::new(headers);
    for item in items {
        table = table.add_row(&vec![
                format!("#{}", item.id),
                item.name,
                format!("{:.2}", item.amount),
                format!("{} days", item.days),
            ]
        );
    }

    let display = table.show().clone();

    Ok(format!("{}", display))
}

pub fn add_item(conn: SqliteConnection) -> std::result::Result<String, Box<dyn error::Error>> {
    let name = cli::ask_str("what would you like the item title to be?")?;
    let amount = cli::ask_f32("how much is this item?")?;
    let days = cli::ask_i32("how many days does this amount cover?")?;

    budgeter::create_item(conn, &name, &days, &amount)?;
    Ok("successfully created item".to_string())
}

pub fn edit_item(conn: SqliteConnection, id: i32) -> std::result::Result<String, Box<dyn error::Error>> {
    let name = cli::ask_str("what would you like the item title to be?")?;
    let amount = cli::ask_f32("how much is this item?")?;
    let days = cli::ask_i32("how many days does this amount cover?")?;
                            
    budgeter::edit_item(conn, id, &name, &days, &amount)?;
    Ok("successfully edited item".to_string())
}

pub fn remove_item(conn: SqliteConnection, id: i32) -> std::result::Result<String, Box<dyn error::Error>> {
    budgeter::delete_item(conn, id)?;
    Ok("successfully removed item".to_string())
}

// pub fn show_budget(duration: i32) {
//     budget::create_for(duration)?
//     Ok("here is budgete")
// }