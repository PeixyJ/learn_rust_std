fn main() {
    println!("Hello, world!");
}

#[test]
fn fn1() {
    let current_column = column!();
    println!("Current column: {:?}", current_column);
}
