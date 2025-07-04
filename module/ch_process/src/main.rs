use std::process::{Command, Stdio};

fn main() {
    println!("Hello, world!");
}

#[test]
fn fn1() {
    let output = Command::new("echo")
        .arg("Hello, world!")
        .output()
        .expect("failed to execute process");
    assert_eq!(b"Hello, world!\n", output.stdout.as_slice());
}
#[test]
fn fn2_io() {
    let echo_child = Command::new("echo")
        .arg("Hello, world!")
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to execute process");

    let echo_out = echo_child.stdout.expect("failed to capture stdout");

    let mut sed_child = Command::new("sed")
        .arg("s/world/universe/")
        .stdin(Stdio::from(echo_out))
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to execute process");
    let sed_out = sed_child
        .wait_with_output()
        .expect("failed to capture stdout");
    assert_eq!(b"Hello, universe!\n", sed_out.stdout.as_slice());
}

#[test]
fn fn3_docker() {
    let _ = Command::new("docker")
        .arg("run")
        .arg("-d")
        .arg("--rm")
        .arg("--name")
        .arg("nginx")
        .arg("nginx:latest")
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to execute docker process");
}
#[test]
fn fn4_stop_docker(){
    let _ = Command::new("docker")
        .arg("stop")
        .arg("nginx")
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to stop docker process");
}