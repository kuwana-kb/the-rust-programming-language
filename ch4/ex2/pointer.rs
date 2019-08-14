fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize { // sはStringへの参照
    s.len() // ここで、sはスコープ外になる。しかし、参照しているものの所有権を持っているわけではないので何も起こらない
}