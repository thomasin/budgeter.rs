// use diesel::table;

table! {
    items (id) {
        id -> Integer,
        name -> Text,
        duration_unit -> Text,
        duration_amount -> Integer,
        cost -> Float,
    }
}
