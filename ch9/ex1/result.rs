use std::fs::File;

fn main() {
    let f = File::open("lib.rs");
    println!("{:#?}", f);

    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error);
        },
    };
}