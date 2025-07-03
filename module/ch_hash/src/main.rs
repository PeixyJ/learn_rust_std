use std::collections::HashMap;
use std::hash::{DefaultHasher, Hash, Hasher};

fn main() {
    println!("Hello, world!");
}

#[derive(Hash, Debug)]
struct User {
    id: u32,
    name: String,
}

#[test]
fn fn1() {
    let user1 = User {
        id: 1,
        name: String::from("Alice"),
    };
    let user2 = User {
        id: 2,
        name: String::from("Bob"),
    };
    assert_ne!(calculate_hash(&user1), calculate_hash(&user2));

    println!("User1 Hash: {:?}", calculate_hash(&user1));
    println!("User2 Hash: {:?}", calculate_hash(&user2));
}

#[test]
fn fn2() {
    let user1 = User {
        id: 1,
        name: String::from("Alice"),
    };
    let user2 = User {
        id: 1,
        name: String::from("Alice"),
    };
    let mut map = HashMap::<String, User>::new();
    map.insert(String::from("user1"), user1);
    map.insert(String::from("user2"), user2);
    println!("Map: {:?}", map);
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}
