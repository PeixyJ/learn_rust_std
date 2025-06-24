use std::ascii::AsciiExt;

#[derive(Debug)]
struct User {
    name: String,
    age: u8,
}

impl Default for User {
    fn default() -> Self {
        User {
            name: String::from("Default Name"),
            age: 0,
        }
    }
}
/**
创建数组
 */
fn fn1() {
    let _: [u8; 2] = [1, 2];
}

/**
创建数组,简化创建数组
 */
fn fn2() {
    let _ = [1, 2];
}

/**
创建自定义对象数组
 */
fn fn3() {
    let tony = User {
        name: String::from("Tony"),
        age: 18,
    };
    let jack = User {
        name: String::from("Jack"),
        age: 20,
    };
    let _: [User; 2] = [tony, jack];
}

/**
解构数组
 */
fn fn4() {
    let tony = User {
        name: String::from("Tony"),
        age: 18,
    };
    let jack = User {
        name: String::from("Jack"),
        age: 20,
    };
    let users = [tony, jack];
    let [tony1, jack2] = users;
    println!("Tony's age: {}, Jack's age: {}", tony1.age, jack2.age);
}
/**
遍历数组
 */
fn fn5() {
    let tony = User {
        name: String::from("Tony"),
        age: 18,
    };
    let jack = User {
        name: String::from("Jack"),
        age: 20,
    };
    let users: [User; 2] = [tony, jack];

    for user in users {
        println!("User name: {}, age: {}", user.name, user.age);
    }
}

/**
数组简单操作
 */
fn fn6() {
    let tony = User {
        name: String::from("Tony"),
        age: 18,
    };
    let jack = User {
        name: String::from("Jack"),
        age: 20,
    };
    let mut users: [User; 2] = [tony, jack];

    // 修改数组元素
    users[0].age = 21;
    println!("Updated Tony's age: {}", users[0].age);

    // 获取数组长度
    println!("Number of users: {}", users.len());

    // 获取数组第一个元素
    if let Some(first_user) = users.get(0) {
        println!("First user: {}, age: {}", first_user.name, first_user.age);
    }

    // 获取数组最后一个元素
    if let Some(last_user) = users.last() {
        println!("Last user: {}, age: {}", last_user.name, last_user.age);
    }

    // 检查数组是否为空
    if users.is_empty() {
        println!("The users array is empty.");
    } else {
        println!("The users array is not empty.");
    }

    // 清空数组
    // 因为 Rust 中的数组大小是固定的，所以不能直接清空数组，但可以通过重新赋值来实现类似效果。

    //排序数组
    users.sort_by(|x, x1| x.age.cmp(&x1.age));
    println!("Users sorted by age:{:?}", users);

    // 查找元素
    let search_name = "Jack";
    if let Some(user) = users.iter().find(|&u| u.name == search_name) {
        println!("Found user: {}, age: {}", user.name, user.age);
    } else {
        println!("User with name {} not found.", search_name);
    }
    // 反转数组
    users.reverse();
    println!("Users after reverse: {:?}", users);
}

/**
将元组转换为数组
 */
fn fn7() {
    let tuple = (0, 1, 2);
    let arr: [i32; 3] = tuple.into();
    println!("Converted array: {:?}", arr);
}

/**
Map操作
 */
#[test]
fn fn9() {
    let arr = [1, 2, 3];
    let arr1 = arr.map(|x| x + 1);
    println!("Converted array: {:?}", arr);
    println!("Converted array: {:?}", arr1);
}

#[test]
fn fn10() {
    let arr = [1, 2, 3];
    let arr1 = &arr[1..];
    let arr2 = arr.as_slice();
    println!("Original array: {:?}", arr1);
}

fn main() {
    fn1();
    fn2();
    fn3();
    fn4();
    fn5();
    fn6();
    fn7();
}
