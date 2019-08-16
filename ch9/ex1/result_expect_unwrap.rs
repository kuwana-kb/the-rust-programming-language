use std::fs::File;

fn main() {
    //let f = File::open("hellox.txt").unwrap();

    let f = File::open("hellox.txt").expect("Failed to open file");
}