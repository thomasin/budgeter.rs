use yansi::Paint;
use std::io::stdin;
use std::io;
use std::error;
use std::cmp::max;
use itertools::{Itertools, EitherOrBoth};
use log;

// Messages

pub fn message(msg: &str) {
    println!("{}", Paint::new(msg).bold());
}

pub fn success(msg: &str) {
    println!("{}", Paint::green(msg).bold());
}

pub fn problem(msg: &str) {
    println!("{}", Paint::red(msg).bold());
}

// Interactivity

pub fn ask_str(msg: &str) -> Result<String, io::Error> {
    message(msg);
    let mut input = &mut String::new();
    stdin().read_line(&mut input)?;
    Ok(input[..(input.len() - 1)].to_string()) // Drop the newline character 
}

pub fn ask_f32(msg: &str) -> Result<f32, Box<dyn error::Error>> {
    let response = ask_str(msg)?;
    let response = response.parse::<f32>()?;
    Ok(response)
}

pub fn ask_i32(msg: &str) -> Result<i32, Box<dyn error::Error>> {
    let response = ask_str(msg)?;
    let response = response.parse::<i32>()?;
    Ok(response)
}

// Tables

pub struct Table {
    headers: Vec<String>, // Any length headers
    lengths: Vec<usize>, // A running count of the max lengths of each col
    rows: Vec<Vec<String>>, // A list of table rows,
}

impl Table {
    pub fn new(headers: Vec<String>) -> Table {
        Table {
            headers: headers.clone(),
            lengths: headers.clone().iter().map(|s| s.len()).collect(),
            rows: Vec::with_capacity(headers.clone().iter().count()),
        }
    }

    pub fn add_row(&self, row: &Vec<String>) -> Table {
        let mut rows = self.rows.clone();
        rows.push(row.clone());

        let new_lengths:Vec<usize> = row.clone().iter().map(|s| s.len()).collect();
        let cloned_l = self.lengths.clone();
        let lengths = cloned_l.iter().zip_longest(new_lengths);
        let summed_lengths = lengths.map(|either| {
            match either {
                EitherOrBoth::Both(a, b) => max(1, max(*a, b)),
                EitherOrBoth::Left(a) => max(1, *a),
                EitherOrBoth::Right(b) => max(1, b),
            }
        });

        Table {
            headers: self.headers.clone(),
            lengths: summed_lengths.collect(),
            rows,
        }
    }

    pub fn show(&self) -> String {
        let mut ascii = self.show_headers();
        for row in &self.rows {
            ascii = ascii + &self.show_row(row.to_vec());
        }
        ascii
    }

    fn show_headers(&self) -> String {
        let mut ascii = "-".repeat(self.table_width());
        ascii += "\n|";
        self.show_row(self.headers.clone())
    }

    fn show_row(&self, row: Vec<String>) -> String {
        let mut ascii = "|".to_string();
        for (pos, col) in row.iter().enumerate() {
            let length = self.lengths[pos];
            ascii = ascii + &format!(" {:1$} |", &col, length);
        }
        ascii.push('\n');
        ascii = ascii + &"-".repeat(self.table_width());
        ascii.push('\n');
        ascii
    }

    fn table_width(&self) -> usize {
        self.lengths.iter().sum::<usize>() + (3 * self.lengths.len()) + 1
    }
}



