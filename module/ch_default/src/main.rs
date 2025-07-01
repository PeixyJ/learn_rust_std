fn main() {
    println!("Hello, world!");
}

struct User {
    name: String,
    age: u32,
}

impl Default for User {
    fn default() -> Self {
        User {
            name: "Default User".to_string(),
            age: 30,
        }
    }
}

#[test]
fn fn1() {
    let user = User::default();
    assert_eq!(user.name, "Default User");
    assert_eq!(user.age, 30);
}
