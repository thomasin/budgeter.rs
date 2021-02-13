use diesel::Insertable;
use diesel::Queryable;
use diesel::dsl::Eq;
use diesel::expression_methods::ExpressionMethods;

use crate::schema::items;
use crate::duration::{Duration};


pub struct Item {
    pub id: i32,
    pub name: String,
    pub duration: Duration,
    pub cost: f32,
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

#[derive(Insertable)]
#[table_name="items"]
pub struct NewItem {
    pub name: String,
    #[diesel(embed)]
    pub duration: Duration,
    pub cost: f32,
}

impl<'insert> Insertable<items::table> for &'insert Duration {
    type Values = <(
        Eq<items::duration_unit, String>,
        Eq<items::duration_amount, i32>,
    ) as Insertable<items::table>>::Values;

    fn values(self) -> Self::Values {
        Insertable::values((
            ExpressionMethods::eq(items::duration_unit, self.unit()),
            ExpressionMethods::eq(items::duration_amount, self.amount())
        ))
    }
}

impl Insertable<items::table> for Duration {
    type Values = <(
        Eq<items::duration_unit, String>,
        Eq<items::duration_amount, i32>,
    ) as Insertable<items::table>>::Values;

    fn values(self) -> Self::Values {
        Insertable::values((
            ExpressionMethods::eq(items::duration_unit, self.unit()),
            ExpressionMethods::eq(items::duration_amount, self.amount())
        ))
    }
}
