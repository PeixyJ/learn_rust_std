use std::any::Any;
use std::fmt::Debug;

fn main() {
    println!("Hello, world!");
    let a = 1;
    let c = String::from("Hello, Rust!");
    log(&a);
    log(&c);
}

fn log<T: Any + Debug>(value: &T) {
    let value_any = value as &dyn Any;
    if let Some(s) = value_any.downcast_ref::<String>() {
        println!("String value: {}", s);
    } else if let Some(i) = value_any.downcast_ref::<i32>() {
        println!("Integer value: {}", i);
    } else {
        println!("Unknown type: {:?}", value);
    }
}
