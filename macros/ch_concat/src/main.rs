fn main() {
    println!("Hello, world!");
}

#[test]
fn fn_concat() {
    let c = concat!("World", "Added", "String");
    println!("Concatenated string: {}", c);
}
