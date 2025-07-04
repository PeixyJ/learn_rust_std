fn main() {
    println!("Hello, world!");
}

#[test]
fn fn1() {
    if cfg!(windows) {
        println!("This is windows :(");
    }
    if cfg!(unix) {
        println!("This is unix :)");
    }
}
#[test]
fn fn2() {
    if cfg!(target_os = "linux") {
        println!("This is linux :)");
    }
    if cfg!(target_os = "macos") {
        println!("This is macos :)");
    }
    if cfg!(target_os = "windows") {
        println!("This is windows :)");
    }
}

#[test]
fn fn3() {
    if cfg!(target_arch = "x86_64") {
        println!("This is x86_64 :)");
    }
    if cfg!(target_arch = "aarch64") {
        println!("This is aarch64 :)");
    }
}
#[test]
fn fn4() {
    if cfg!(feature = "feature1") {
        println!("Feature 1 is enabled :)");
    } else {
        println!("Feature 1 is not enabled :(");
    }
}

#[test]
fn fn5() {
    if cfg!(debug_assertions) {
        println!("Debug assertions are enabled :)");
    } else {
        println!("Debug assertions are not enabled :(");
    }
}

#[test]
fn fn6() {
    if cfg!(test) {
        println!("This is a test build :)");
    } else {
        println!("This is not a test build :(");
    }
}
