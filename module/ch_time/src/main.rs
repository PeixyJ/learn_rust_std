use std::time::{Duration, SystemTime};

fn main() {
    println!("Hello, world!");
}

#[test]
fn fn_duration_1() {
    let five_seconds = Duration::from_secs(5);
    assert_eq!(five_seconds, Duration::from_millis(5_000));
    let ten_millis = Duration::from_millis(10);
    let seven_nano = Duration::from_nanos(7_000_000);
    let total = five_seconds + ten_millis + seven_nano;
    assert_eq!(total, Duration::from_millis(5_017));
}

#[test]
fn fn_system_time() {
    let now = SystemTime::now();
    println!("Current system time: {:?}", now);
}
