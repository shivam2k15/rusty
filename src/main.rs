//moving ownership borrowing references
fn main() {
    let mut a = String::from("some");
    // let b = a;
    check(&mut a);
    println!("{}", a);
}

fn check(s: &mut String) {
    s.push_str("string");
    println!("{}", s);
}
