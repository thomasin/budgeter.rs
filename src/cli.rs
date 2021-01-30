use yansi::Paint;

pub fn message(msg: String) {
    println!("{}", Paint::new(msg).bold());
}

pub fn success(msg: String) {
    println!("{}", Paint::green(msg).bold());
}

pub fn problem(msg: String) {
    println!("{}", Paint::red(msg).bold());
}
