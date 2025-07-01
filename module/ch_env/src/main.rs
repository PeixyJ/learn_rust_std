use std::env;
use std::env::{args, args_os, current_dir, current_exe, home_dir, set_current_dir, split_paths, temp_dir, var, var_os, vars};
use std::path::Path;

//./target/debug/ch_env abc
fn main() {
    let args = args();
    for arg in args {
        println!("Argument: {}", arg);
    }
}

#[test]
fn fn1() {
    let args = args_os();
    for arg in args {
        println!("Argument: {:?}", arg);
    }
}
#[test]
fn fn2() {
    let current = match current_dir() {
        Ok(path) => path,
        Err(e) => {
            eprintln!("Error getting current directory: {}", e);
            return;
        }
    };
    let current_path = current.to_str().unwrap();
    println!("Current directory as string: {}", current_path);
    println!("Current directory: {}", current.display());
}
#[test]
fn fn3() {
    let current_exe = match current_exe() {
        Ok(path) => path,
        Err(e) => {
            eprintln!("Error getting current executable: {}", e);
            return;
        }
    };
    let current_exe_str = current_exe.to_str().unwrap();
    println!("Current executable as string: {}", current_exe_str);
    println!("Current executable: {}", current_exe.display());
}

#[test]
fn fn4() {
    let home = home_dir().unwrap();
    let home_str = home.to_str().unwrap();
    println!("Home directory as string: {}", home_str);
}

#[test]
fn fn5() {
    let path = Path::new("/tmp");
    match set_current_dir(path) {
        Ok(_) => {
            println!("Changed current directory!");
        }
        Err(err) => {
            eprintln!("Failed to change current directory: {}", err);
        }
    }
    let current = match current_dir() {
        Ok(path) => path,
        Err(e) => {
            eprintln!("Error getting current directory: {}", e);
            return;
        }
    };
    println!("Current directory as string: {}", current.display());
}

#[test]
fn fn6() {
    let current = match current_dir() {
        Ok(path) => path,
        Err(e) => {
            eprintln!("Error getting current directory: {}", e);
            return;
        }
    };
    let paths = split_paths(&current);
    for path in paths {
        println!("Path: {}", path.display());
    }
}
#[test]
fn fn7() {
    match set_current_dir(temp_dir()) {
        Ok(_) => {
            println!("Changed current directory to temp directory!");
            println!("Temp directory: {}", current_dir().unwrap().display());
        }
        Err(err) => {
            eprintln!("Failed to change current directory: {}", err);
        }
    }
}

#[test]
fn fn8() {
    let var = var("HOME").unwrap();
    println!("HOME environment variable: {}", var);
}

#[test]
fn fn9(){
    let os = var_os("HOME").unwrap();
    println!("HOME environment variable (os): {:?}", os);
}

#[test]
fn fn10(){
    let vars = vars();
    for (key, value) in vars {
        println!("{}: {}", key, value);
    }
}