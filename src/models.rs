use super::schema::items;

#[derive(Queryable)]
pub struct Item {
    pub id: i32,
    pub name: String,
    pub days: i32,
    pub amount: f32,
}

#[derive(Insertable)]
#[table_name="items"]
pub struct NewItem<'a> {
    pub name: &'a str,
    pub days: &'a i32,
    pub amount: &'a f32,
}
