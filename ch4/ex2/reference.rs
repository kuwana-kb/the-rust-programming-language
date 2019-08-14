fn main() {
    let mut s = String::from("hello");

    change(&mut s);

    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;

    let mut s1 = String::from("hello");

    {
        let r3 = &mut s1;
    } // r3はここでスコープを抜けるので問題なく新しい参照を作ることができる
    let r4 = &mut s1;
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}