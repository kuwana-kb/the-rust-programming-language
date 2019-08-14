fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // wordの中身は、値5になる
    println!("{}", s);
    println!("{}",word);

    s.clear(); // Stringを空にする。つまり、""と等しくする

    // wordはまだ値5を保持しているが、もうこの値を有効に活用できる文字列は存在しない
    // wordは完全に無効なのだ！
}