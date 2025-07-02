fn main() {
    println!("Hello, world!");
    fn1(6);
}

//q: 如何实现一个自定义的 Error
// a: 可以通过实现 std::error::Error trait 来创建自定义错误类型。
use std::error::Error;
use std::fmt;
#[derive(Debug)]
struct MyError {
    details: String,
}
impl MyError {
    fn new(msg: &str) -> MyError {
        MyError {
            details: msg.to_string(),
        }
    }
}
impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MyError: {}", self.details)
    }
}

impl Error for MyError {
    fn description(&self) -> &str {
        &self.details
    }
}

fn fn1(input: i32) {
    if input >= 5 {
        panic!("input is greater than or equal to 5");
    } else {
        println!("input is less than 5");
    }
}

#[test]
fn fn2() {
    let path = std::env::var("IMPORTANT_PATH").expect("Missing IMPORTANT_PATH env var");
}

#[test]
// q: 如何在函数中返回一个自定义的错误
// a: 可以使用 Result 类型来返回自定义错误。
// 这里的 Result<T, E> 中 T 是成功时的返回类型，E 是错误类型。
fn fn3() -> Result<(), MyError> {
    // 模拟一个错误
    Err(MyError::new("oh no!"))
}
