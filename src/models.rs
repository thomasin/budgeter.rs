use diesel::Queryable;
use crate::schema::items;
use crate::duration::{Duration};

pub struct Item {
    pub id: i32,
    pub name: String,
    pub duration: Duration,
    pub cost: f32,
}

#[derive(Insertable)]
#[table_name="items"]
pub struct NewItem<'a> {
    pub name: &'a str,
    pub duration_unit: &'a str,
    pub duration_amount: &'a i32,
    pub cost: &'a f32,
}

impl Queryable<items::SqlType, diesel::sqlite::Sqlite> for Item {
    type Row = (i32, String, String, i32, f32);

    fn build(row: Self::Row) -> Self {
        let duration: Duration = match &row.2[..] {
            "M" => Duration::Month(row.3),
            "D" => Duration::Day(row.3),
            _ => Duration::Day(row.3),
        };

        Item {
            id: row.0,
            name: row.1,
            duration,
            cost: row.4,
        }
    }
}
