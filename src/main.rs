// using package manager

use chrono::{Local, Utc};

fn main() {
    let first = Local::now();
    let second = Utc::now();
    println!("{}", first);
    println!("{}", second.format("%d-%m-%Y - %H:%M:%S"));
}
