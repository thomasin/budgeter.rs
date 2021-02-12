#[derive(Debug, Clone, Copy)]
pub enum Duration {
    Day(i32),
    Month(i32),
}

impl Duration {
    pub fn humanise(&self) -> String {
        match &self {
            Duration::Day(days) => {
                if days == &14 { "fortnight".to_string() }
                else if days % 7 == 0 { pluralise(&(days / 7), "week") }
                else { pluralise(days, "day") }
            }
            Duration::Month(months) => {
                if months % 12 == 0 { pluralise(&(months / 12), "year") }
                else { pluralise(months, "month") }
            }
        }
    }
}

pub fn pluralise(amount: &i32, unit: &str) -> String {
    match amount {
        1 => format!("1 {}", unit),
        _ => format!("{} {}s", amount, unit),
    }
}

// pub fn days_in_month<T: TimeZone>(date: Date<T>) -> i32 {
//     match date.month() {
//         1  => 31,
//         2  => {
//             // https://en.wikipedia.org/wiki/Leap_year#Algorithm
//             let year = date.year();
//             if (year % 4) > 0 { 28 }
//             else if (year % 100) > 0 { 29 }
//             else if (year % 400) > 0 { 28 }
//             else { 29 }
//         },
//         3  => 31,
//         4  => 30,
//         5  => 31,
//         6  => 30,
//         7  => 31,
//         8  => 31,
//         9  => 30,
//         10 => 31,
//         11 => 30,
//         12 => 31,
//         _ => panic!("impossible date"),
//     }
// }
