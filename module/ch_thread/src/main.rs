use std::thread;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    println!("Hello, world!");
}

#[test]
fn fn_thread_ch_1() {
    let nt = thread::spawn(move || {
        sleep(Duration::new(5, 0));
        println!("Hello from a thread!");
    });
    println!("Hello from out thread!");
    nt.join().unwrap();
}

#[test]
fn fn_thread_ch_2() {
    let builder = thread::Builder::new()
        .name("test_thread".to_string());
    let handler = builder
        .spawn(|| {
        sleep(Duration::new(2, 0));
            println!("Hello from a thread!");
        })
        .unwrap();
    println!("Thread spawned with name: {}", handler.thread().name().unwrap());
    println!("Thread ID: {:?}", handler.thread().id());
    handler.join().unwrap();
}
