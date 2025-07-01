fn main() {
    println!("Hello, world!");
}

#[test]
fn fn1() {
    let a = 'A';
    let b = '🐲';
    println!("a = {}, b = {}", a.is_ascii(), b.is_ascii());
}
