use std::fmt::Error;

fn main() {
    println!("Hello, world!");
}

struct User {
    id: u32,
    name: String,
}

struct Member {
    id: u32,
    name: String,
    role: String,
}
//使用 Member 转 User
impl From<Member> for User {
    fn from(value: Member) -> Self {
        User {
            id: value.id,
            name: value.name,
        }
    }
}


#[test]
fn convert_member_to_user() {
    let member = Member {
        id: 1,
        name: "Alice".to_string(),
        role: "Admin".to_string(),
    };
    let user = User::from(member);
    println!("User ID: {}, Name: {}", user.id, user.name);
}
