use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hellox.txt")?;
    let mut s = String::new();

    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file_short() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hellox.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

fn main() {
    let r = read_username_from_file();
    match r {
        Ok(v) => println!("Ok, {}", v),
        Err(e) => println!("Err, {}", e),
    }

    let r = read_username_from_file_short();
    match r {
        Ok(v) => println!("Ok short: {}", v),
        Err(e) => println!("Err short: {}", e),
    }
}