fn main() {
    println!("Hello, world!");
}

#[test]
fn fn1() {
    let my_string = include!("../monkeys.in");
    println!("Included string: {}", my_string);
}
