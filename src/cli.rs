use yansi::Paint;

pub fn message(msg: &str) {
    println!("{}", Paint::new(msg).bold());
}

pub fn success(msg: &str) {
    println!("{}", Paint::green(msg).bold());
}

pub fn problem(msg: &str) {
    println!("{}", Paint::red(msg).bold());
}
