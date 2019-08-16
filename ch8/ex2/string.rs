fn main() {
    let mut s = String::new();

    let data = "initial contents";
    println!("{}", data);

    let s = data.to_string();
    println!("{}", s);

    let s = "initial contents".to_string();
    println!("{}", s);

    let s = String::from("initial contents");
    println!("{}", s);

    let mut s = "foo".to_string();
    s.push_str("bar");
    println!("{}", s);

    let mut s = String::from("foo");
    let s2 = "bar";
    s.push_str(s2);
    println!("{}", s2);

    let mut s = String::from("lo");
    s.push('l');
    println!("{}", s);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    println!("{}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s4 = s1 + "-" + &s2 + "-" + &s3;
    println!("{}", s4);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s4 = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s4);
}