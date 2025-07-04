fn main() {
    println!("Hello, world!");
}

#[test]
fn fn1() {
    let var = env!("CARGO_PKG_NAME");
    println!("CARGO_PKG_NAME: {}", var);
}
