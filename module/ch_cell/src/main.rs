use std::cell::Cell;

fn main() {
    println!("Hello, world!");
}

struct User {
    name: String,
    login_count: u32,
}

struct MangeUser {
    name: String,
    login_count: Cell<u32>,
}

#[test]
fn fn1() {
    // 从栈移动到堆
    let mut user = User {
        name: String::from("Alice"),
        login_count: 1,
    };

    user.login_count += 1; // 修改结构体中的字段
    println!("User: {}, Login Count: {}", user.name, user.login_count);

    // 使用 Cell 来存储可变数据
    let manage_user = MangeUser {
        name: String::from("Bob"),
        login_count: Cell::new(1),
    };
    manage_user
        .login_count
        .set(manage_user.login_count.get() + 1); // 修改 Cell 中的值
    println!(
        "ManageUser: {}, Login Count: {}",
        manage_user.name,
        manage_user.login_count.get()
    );
}
