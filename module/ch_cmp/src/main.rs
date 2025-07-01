use std::cmp;
fn main() {
    println!("Hello, world!");
}

struct User {
    name: String,
    age: u32,
}

impl PartialEq<Self> for User {
    fn eq(&self, other: &Self) -> bool {
        &self.age == &other.age
    }
}

impl PartialOrd<Self> for User {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.age.cmp(&other.age))
    }
}

#[test]
fn fn1() {
    let x = 5;
    let y = 6;
    let max = cmp::max(x, y);
    println!("max = {}", max);

    let tony = User {
        name: String::from("Alice"),
        age: 30,
    };

    let bob = User {
        name: String::from("Bob"),
        age: 30,
    };

    let job = User {
        name: String::from("Charlie"),
        age: 25,
    };

    println!("The same age? {} ", tony == bob);
    println!("Is Alice older than Bob? {} ", tony > job);
}

#[test]
fn fn2() {
    let tony = User {
        name: String::from("Alice"),
        age: 30,
    };

    let job = User {
        name: String::from("Charlie"),
        age: 25,
    };
    let old_user = cmp::max_by(tony, job, |a, b| a.age.cmp(&b.age));
    println!("The older user is: {} with age {}", old_user.name, old_user.age);
}

fn fn3() {
    let tony = User {
        name: String::from("Alice"),
        age: 30,
    };

    let job = User {
        name: String::from("Charlie"),
        age: 25,
    };
}
