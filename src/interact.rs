use diesel::SqliteConnection;
use crate::duration::{Duration};

use std::error;

use crate::cli;

// This module might go well in main.rs instead

pub fn list_all_items(conn: SqliteConnection) -> std::result::Result<(), Box<dyn error::Error>> {
    let items = super::libr::list_items(conn)?;

    let mut table = cli::Table::new(vec![
        "id".to_string(),
        "name".to_string(),
        "cost".to_string(),
        "currency".to_string(),
        "over".to_string(),
    ]);

    for item in items {
        table.add_row(vec![
            format!("#{}", item.id),
            item.name,
            format!("{:.2}", item.cost),
            item.currency,
            item.duration.humanise().to_string(),
        ]);
    }

    table.show();

    Ok(())
}

pub fn add_item(conn: SqliteConnection) -> std::result::Result<(), Box<dyn error::Error>> {
    let name = cli::ask_str("what would you like the item title to be?")?;
    let cost = cli::ask_f32("how much is this item?")?;
    let currency = cli::ask_str("what currency is this item cost in?")?;

    let duration_str = cli::ask_str("what time period does this amount cover?")?;
    let duration = Duration::from_string(duration_str)?;

    super::libr::create_item(conn, name, duration, cost, currency)?;
    cli::success("✨ created item");
    Ok(())
}

pub fn edit_item(conn: SqliteConnection, id: i32) -> std::result::Result<(), Box<dyn error::Error>> {
    let name = cli::ask_str("what would you like the item title to be?")?;
    let cost = cli::ask_f32("how much is this item?")?;
    let currency = cli::ask_str("what currency is this item cost in?")?;

    let duration_str = cli::ask_str("what time period does this amount cover?")?;
    let duration = Duration::from_string(duration_str)?;

    super::libr::edit_item(conn, id, name, duration, cost, currency)?;
    cli::success("✨ edited item");
    Ok(())
}

pub fn remove_item(conn: SqliteConnection, id: i32) -> std::result::Result<(), Box<dyn error::Error>> {
    super::libr::delete_item(conn, id)?;
    cli::success("✨ removed item");
    Ok(())
}

pub fn show_budget(conn: SqliteConnection) -> std::result::Result<(), Box<dyn error::Error>> {
    let duration_str = cli::ask_str("what time period should this budget cover?")?;
    let duration = Duration::from_string(duration_str)?;

    let budget = crate::libr::show_budget(conn, duration)?;

    for (currency, budget) in budget.split_by_currency() {
        println!("total {}: {:.2}", currency, budget.total());
    }

    Ok(())
}
