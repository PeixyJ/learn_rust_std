use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, BufWriter, Read, Seek, SeekFrom, Write};
use std::time::SystemTime;

fn main() {
    println!("Hello, world!");
}

#[test]
fn fn1() {
    let mut f = File::options()
        .write(true)
        .read(true)
        .open("dir.txt")
        .expect("Could not open file");
    f.write_all(b"Hello, world!").unwrap();
    let mut bf = [0; 10];
    f.seek(SeekFrom::Start(0)).unwrap();
    let n = f.read(&mut bf).unwrap();
    println!("The bytes read: {:?}", &bf[..n]);
}

#[test]
fn fn2() {
    let f = File::open("dir.txt").expect("Could not open file");
    let mut reader = BufReader::new(f);
    let mut buffer = String::new();
    reader.read_line(&mut buffer).expect("Could not read file");
    assert_eq!(buffer, "Hello, world!");
}

#[test]
fn fn3() {
    let f = File::options()
        .write(true)
        .read(true)
        .create(true)
        .open("dir.txt")
        .expect("Could not open file");
    let mut writer = BufWriter::new(f);
    let time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();
    let current_time = format!("Current time: {}", time);
    writer
        .write(current_time.as_bytes())
        .expect("Could not write to file");
    writer.flush().unwrap()
}

#[test]
fn fn4() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    println!("The input is: {}", input);
    Ok(())
}

#[test]
fn fn5() -> io::Result<()> {
    io::stdout().write(b"Hello, world!")?;
    Ok(())
}

#[test]
fn fn6() -> io::Result<()> {
    let f = File::open("dir.txt")?;
    let mut reader = BufReader::new(f);
    for line in reader.lines() {
        let line = line?;
        println!("Line: {}", line);
    }
    Ok(())
}
