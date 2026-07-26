// find char 'a' index in a string application if option

fn main() {
    let s = String::from("Shivam");
    let index = find_index(s, 'k');
    match index {
        Some(index) => print!("{}", index),
        None => print!("no index"),
    }
}

fn find_index(s: String, a: char) -> Option<usize> {
    for (index, character) in s.chars().enumerate() {
        if character == a {
            return Some(index);
        }
    }
    return None;
}
