use std::fmt::format;

fn main() {
    println!("Hello, world!");
}

#[test]
fn fn1() {
    assert_eq!(format!("{}", "Hello, world!"), "Hello, world!");
    assert_eq!(format!("{:?}", "Hello, world"), "\"Hello, world\"");

    println!("{:?}", "This is a test function".to_string());
    println!("{name}", name = "peixy");
    println!("{name} {name}", name = "peixy");
    let tony = "tony";
    println!("{tony} {tony}");
    println!("{:04}", 42);
    println!("{:#?}", (42, "hello", true));
}

#[test]
fn fn2() {
    println!("{1},{0}", "hello", "world");
}

#[test]
fn fn3() {
    println!("Hello, {name}!", name = "peixy");
    println!("{name} {}", 1, name = "peixy",);
    println!("{a} {b}", a = "peixy", b = "tony");
}

#[test]
fn fn4() {
    let a = 1;
    let b = &a;
    println!("{a:e} {b:p}")
}

#[test]
fn fn5() {
    println!("Hello, {:5}!", "x");
    println!("Hello, {:1$}!", "x", 5);
    println!("Hello, {1:0$}!", 5, "x");
    println!("Hello, {:x<5}!", "P");
}

#[test]
fn fn6() {
    assert_eq!(format!("{0:.5}", 0.1), "0.10000");
}

#[test]
fn fn7() {
    assert_eq!(format!("{{"), "{");
}
