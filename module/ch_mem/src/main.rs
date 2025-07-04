fn main() {
    println!("Hello, world!");
}
struct User {
    
    name: String,
    age: u8,
    
}

#[test]
fn fn1() {
    let user = User {
        name: String::from("Alice"),
        age: 30,
    };
    println!("User name: {}, age: {}", user.name, user.age);
    println!("User Memory Address: {:p}", &user);
    println!("User Memory Size: {}", size_of::<User>());
}
