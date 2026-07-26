// read a file text or return error via result enum
use std::fs::read_to_string;
fn main() {
    let result = read_to_string("a.lock");
    match result {
        Ok(index) => print!("{}", index),
        Err(err) => panic!("Error: {}", err),
    }
}
