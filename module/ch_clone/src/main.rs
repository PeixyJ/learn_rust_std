fn main() {
    println!("Hello, world!");
}

#[derive(Clone)]
struct User {
    name: String,
    age: u8,
}

#[test]
fn fn1() {
    let s = String::from("hello");
    let s1 = s.clone(); // 克隆字符串
    println!("s: {}, s1: {}", s, s1);

    let tony = User {
        name: String::from("Tony"),
        age: 30,
    };

    let tony_clone = tony.clone(); // 克隆结构体

    println!("Tony: {}, Age: {}", tony.name, tony.age);
    println!(
        "Tony Clone: {}, Age: {}",
        tony_clone.name, tony_clone.age
    );
}
