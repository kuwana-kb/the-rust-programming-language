use std::sync::mpsc;
use std::thread;

fn main() {
    //multiple producer single consumer
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val =  String::from("hi");
        tx.send(val).unwrap();
        // println!("val is {}", val); // value borrowed error
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
