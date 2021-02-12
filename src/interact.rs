use diesel::SqliteConnection;

use std::error;

use crate::cli;

pub fn list_all_items(conn: SqliteConnection) -> std::result::Result<(), Box<dyn error::Error>> {
    let items = super::libr::list_items(conn)?;

    let mut table = cli::Table::new(vec![
        "id".to_string(),
        "name".to_string(),
        "cost".to_string(),
        "period".to_string(),
    ]);

    for item in items {
        table.add_row(vec![
            format!("#{}", item.id),
            item.name,
            format!("{:.2}", item.cost),
            item.duration.humanise().to_string(),
        ]);
    }

    table.show();

    Ok(())
}

pub fn add_item(conn: SqliteConnection) -> std::result::Result<(), Box<dyn error::Error>> {
    let name = cli::ask_str("what would you like the item title to be?")?;
    let cost = cli::ask_f32("how much is this item?")?;

    // To-do: implement duration from humanised input
    let duration_unit = cli::ask_str("enter D for cost over days, M for cost over months")?;
    let duration_amount = cli::ask_i32("how many _ does this amount cover?")?;

    super::libr::create_item(conn, &name, &duration_unit, &duration_amount, &cost)?;
    cli::success("✨ created item");
    Ok(())
}

pub fn edit_item(conn: SqliteConnection, id: i32) -> std::result::Result<(), Box<dyn error::Error>> {
    let name = cli::ask_str("what would you like the item title to be?")?;
    let cost = cli::ask_f32("how much is this item?")?;
    let duration_amount = cli::ask_i32("how many days does this amount cover?")?;
                            
    super::libr::edit_item(conn, id, &name, &duration_amount, &cost)?;
    cli::success("✨ edited item");
    Ok(())
}

pub fn remove_item(conn: SqliteConnection, id: i32) -> std::result::Result<(), Box<dyn error::Error>> {
    super::libr::delete_item(conn, id)?;
    cli::success("✨ removed item");
    Ok(())
}

pub fn show_budget(conn: SqliteConnection, duration: i32) -> std::result::Result<(), Box<dyn error::Error>> {
    // let date = Local::today();
    // let days = duration::days_in_month(date);
    let total = super::libr::show_budget(conn, duration)?;
    println!("total: {:.2}", total);
    Ok(())
}