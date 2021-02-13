#[derive(Debug, Clone, Copy)]
pub enum Duration {
    Day(i32),
    Month(i32),
}

impl Duration {
    pub fn from_string(s: String) -> std::result::Result<Duration, Box<dyn std::error::Error>> {
        match s.split_whitespace().collect::<Vec<&str>>()[..] {
            ["day"] => Ok(Duration::Day(1)),
            ["week"] => Ok(Duration::Day(7)),
            ["fortnight"] => Ok(Duration::Day(14)),
            ["month"] => Ok(Duration::Month(1)),
            ["year"] => Ok(Duration::Month(12)),

            [amount, "day"] => {
                let num = to_number(amount)?;
                Ok(Duration::Day(num))
            },

            [amount, "days"] => {
                let num = to_number(amount)?;
                Ok(Duration::Day(num))
            },

            [amount, "week"] => {
                let num = to_number(amount)?;
                Ok(Duration::Day(num * 7))
            },

            [amount, "weeks"] => {
                let num = to_number(amount)?;
                Ok(Duration::Day(num * 7))
            },

            [amount, "fortnight"] => {
                let num = to_number(amount)?;
                Ok(Duration::Day(num * 14))
            },

            [amount, "fortnights"] => {
                let num = to_number(amount)?;
                Ok(Duration::Day(num * 14))
            },

            [amount, "month"] => {
                let num = to_number(amount)?;
                Ok(Duration::Month(num))
            },

            [amount, "months"] => {
                let num = to_number(amount)?;
                Ok(Duration::Month(num))
            },

            [amount, "year"] => {
                let num = to_number(amount)?;
                Ok(Duration::Month(num * 12))
            },

            [amount, "years"] => {
                let num = to_number(amount)?;
                Ok(Duration::Month(num))
            },

            _ => Err("dont understand this time".into()),
        }
    }

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

    pub fn unit(&self) -> String {
        match &self {
            Duration::Day(_) => "D".to_string(),
            Duration::Month(_) => "M".to_string(),
        }
    }

    pub fn amount(&self) -> i32 {
        match &self {
            Duration::Day(days) => *days,
            Duration::Month(months) => *months,
        }
    }
}

pub fn pluralise(amount: &i32, unit: &str) -> String {
    match amount {
        1 => format!("1 {}", unit),
        _ => format!("{} {}s", amount, unit),
    }
}

pub fn to_number(s: &str) -> std::result::Result<i32, Box<dyn std::error::Error>> {
    match s {
        "a" => Ok(1),
        "one" => Ok(1),
        "two" => Ok(2),
        "three" => Ok(3),
        "four" => Ok(4),
        "five" => Ok(5),
        "six" => Ok(6),
        "seven" => Ok(7),
        "eight" => Ok(8),
        "nine" => Ok(9),
        "ten" => Ok(10),
        num => Ok(num.parse::<i32>()?),
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
