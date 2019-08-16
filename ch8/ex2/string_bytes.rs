fn main() {
    let len = String::from("Hola").len();
    println!("{}", len);

    let len = String::from("こんにちは").len();
    println!("{}", len);

    let hello = String::from("こんにちは");
    let s = &hello[0..6];
    println!("{}", s);

    for c in "こんにちは".chars() {
        println!("{}", c);
    }

    for b in "こんにちは".bytes() {
        println!("{}", b);
    }
}