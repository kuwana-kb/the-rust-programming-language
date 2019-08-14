fn main() {
    {
        let s = String::from("hello"); // sはここから有効になる
        println!("{}", s);
    } // sのスコープはここでおしまい

    let x = 5;
    let y = x;

    let s1 = String::from("hello");
    let s2 = s1;

    println!("{}, world", s1); // error: use of moved value `s1`;

    let s3 = String::from("hello");
    let s4 = s3.clone();

    println("s1 = {}, s2 = {}", s1, s2);
}