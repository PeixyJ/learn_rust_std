fn main() {
    println!("Hello, world!");
}

#[test]
fn fn1() {
    format!("test"); // => "test"
    format!("hello {}", "world!"); // => "hello world!"
    format!("x = {}, y = {val}", 10, val = 30); // => "x = 10, y = 30"
    let (x, y) = (1, 2);
    format!("{x} + {y} = 3"); // => "1 + 2 = 3"
}
