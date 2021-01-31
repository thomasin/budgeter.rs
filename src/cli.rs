use yansi::Paint;
use std::io::stdin;
use std::io;
use std::error;
use std::cmp::max;
use itertools::{Itertools, EitherOrBoth};


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

/*
lets talk about ownership.
ideally, the table struct would own its own properties instead of borrowing them.
i want the properties to be private, so once the table has the properties they cant
be used again. that seems to indicated the parameters of `new` and `add_row` should
_not_ be borrowed. i just want to move ownership into table.
*/

impl Table {
    pub fn new(headers: Vec<String>) -> Table {
        /*
        so then why do we need to clone header here? this is because when
        you assign Table.headers, the _original_ headers that was
        passed into the function is moved to Table.headers and is
        _not available_ for use in the Table.lengths and Table.rows properties.
        it raises a "does not implement Copy" error because if it _did_
        implement copy (like say, an integer) it would copy it behind the
        scenes.
        */
        Table {
            headers: headers.clone(),
            lengths: Table::column_lengths(&headers),
            rows: Vec::with_capacity(headers.clone().iter().count()),
        }
    }

/*
attack of the clones.
this is the same situation as before. we don't want row to be used
after it is passed into Table, so we are not accepting a reference.
mutability of properties is inherited from the struct.
*/

    pub fn add_row(&mut self, row: Vec<String>) -> () {
        self.rows.push(row.clone()); // Add row to mutable self

        // variable shadowing is so helpful here for some
        // cleaner looking code.
        let lengths = Table::column_lengths(&row);
        let lengths = self.lengths.iter().zip_longest(lengths);

        self.lengths = lengths.map(|either| {
            match either {
                // I don't understand why a is a reference here
                EitherOrBoth::Both(a, b) => max(1, max(*a, b)),
                EitherOrBoth::Left(a) => max(1, *a),
                EitherOrBoth::Right(b) => max(1, b),
            }
        }).collect();
    }

    pub fn show(&self) -> () {
        self.show_headers();

        for row in &self.rows {
            self.show_row(row);
        }
    }

    fn show_headers(&self) -> () {
        self.hr("-");
        self.show_row(&self.headers);
    }

    fn show_row(&self, row: &Vec<String>) -> () {
        print!("|");

        for (pos, col) in row.iter().enumerate() {
            print!(" {:1$} |", col, self.lengths[pos]);
        }

        println!("");
        self.hr("-");
    }

    fn hr(&self, chr: &str) -> () {
        let hr = chr.repeat(self.table_width());
        println!("{}", hr);
    }

    fn table_width(&self) -> usize {
        self.lengths.iter().sum::<usize>() + (3 * self.lengths.len()) + 1
    }

    /*
    here we borrow the row. calling .iter() creates an iterator that just
    holds references to the original items in the vector.
    */
    fn column_lengths(row: &Vec<String>) -> Vec<usize> {
        row.iter().map(|c: &String| c.len()).collect()
    }
}



