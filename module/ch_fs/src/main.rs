use std::any::Any;
use std::error::Error;
use std::fs::{read_dir, DirBuilder, File, FileTimes};
use std::io;
use std::io::{read_to_string, Read, Seek, SeekFrom, Write};
use std::ops::Add;
use std::path::Path;
use std::time::{Duration, SystemTime};

fn main() {
    println!("Hello, world!");
}

#[test]
fn fn_structs_0() {
    match DirBuilder::new().create(Path::new("dir")) {
        Ok(_) => {}
        Err(e) => {
            // Handle the error
            println!("Error creating directory: {}", e);
        }
    }
}

/*
 * 简化业务处理
 */
#[test]
fn fn_structs_1() {
    if let Err(e) = DirBuilder::new().create(Path::new("dir")) {
        println!("Error creating directory: {}", e);
    }
}

#[test]
fn fn_read_dir() {
    let x = match read_dir(Path::new("dir")) {
        Ok(dir) => {
            for x in dir {
                match x {
                    Ok(entry) => {
                        // Process the entry
                        println!("Found entry: {:?}", entry.path());
                    }
                    Err(e) => {
                        // Handle the error for this entry
                        println!("Error reading entry: {}", e);
                    }
                }
            }
        }
        Err(e) => {
            // Handle the error
            println!("Error reading directory: {}", e);
            return;
        }
    };
}

#[test]
fn fn_create_file() {
    let mut b = File::create(Path::new("dir.txt")).unwrap();
    b.write(b"Hello, world!").unwrap();
}

#[test]
fn fn_read_file() {
    let mut file = File::open("dir.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    assert_eq!("Hello, world!", contents);
    println!("File contents: {}", contents);
}

#[test]
fn fn_read_buffered() {
    let file = File::open("dir.txt").unwrap();
    let mut reader = io::BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents).unwrap();
    assert_eq!("Hello, world!", contents);
}

#[test]
fn fn_read_to_vec() {
    let mut file = File::open("vec.txt").unwrap();
    let mut contents: Vec<u8> = vec![];
    file.read_to_end(contents.as_mut()).unwrap();
    println!("File contents: {:?}", contents);
}

#[test]
#[should_panic(expected = "File not found")]
fn fn_only_create_new_file() {
    let mut file = File::create("dir.txt").unwrap();
    let mut file = File::create_new("dir.txt").unwrap();
    file.write_all(b"Hello, world!").unwrap();
}

#[test]
fn fn_options() {
    let mut file = File::options()
        .read(true)
        .write(true)
        .create(true)
        .open("options.txt")
        .unwrap();
    file.write_all(b"Hello, world!").unwrap();

    //写入后文件指针在最后位置所以导致读取不到,需要将 seek 指向开头
    let mut contents = String::new();
    file.seek(SeekFrom::Start(0)).unwrap();
    file.read_to_string(&mut contents).unwrap();
    assert_eq!("Hello, world!", contents);
    println!("File contents: {}", contents);
    //也可以直接读取文件
    let contents = std::fs::read_to_string("options.txt").unwrap();
    assert_eq!("Hello, world!", contents);
}

#[test]
fn fn_file_times() {
    let mut f = File::open("dir.txt").unwrap();
    let ft = FileTimes::new().set_modified(SystemTime::now().add(Duration::from_secs(60)));
    f.set_times(ft).unwrap();
    f.flush().unwrap();
}

#[test]
fn fn_file_metadata() {
    let metadata = File::open("dir.txt").unwrap().metadata().unwrap();
    println!("File size: {}", metadata.len());
    println!("Is file a directory? {}", metadata.is_dir());
    println!("Is file a regular file? {}", metadata.is_file());
    println!("File permissions: {:?}", metadata.permissions());
    println!("Is file a file? {}", metadata.is_file());
    println!("Accessed at: {:?}", metadata.accessed().unwrap());
    println!("Modified at: {:?}", metadata.modified().unwrap());
    println!("Created at: {:?}", metadata.created().unwrap());
}
