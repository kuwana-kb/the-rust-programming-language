use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hellox.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e)
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn main() {
    match read_username_from_file() {
        Ok(result) => println!("OK, {}", result),
        Err(e) => println!("{}", e),
    };

    if let Ok(v) = read_username_from_file() {
        println!("ok2 {}", v);
    }
    if let Err(e) = read_username_from_file() {
        println!("err {}", e);
    }

}