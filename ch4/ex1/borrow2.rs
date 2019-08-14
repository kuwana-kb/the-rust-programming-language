fn main() {
    let s1 = gives_ownership(); // gives_ownershipは戻り値をs1にムーブする
    println!("{}", s1);

    let s2 = String::from("hello"); // s2がスコープに入る

    let s3 = takes_and_gives_back(s2); //s2はtakes_and_gives_backにムーブされ戻り値もs3にムーブされる
    println!("{}", s3);
    println!("{}", s1);
    //println!("{}", s2); // error: borrow of moved value

} // ここでs3はスコープを抜け、ドロップされる。s2もスコープを抜けるが、ムーブされているので何も起きない。s1もスコープを抜けてドロップされる


// gives_ownershipは戻り値を呼び出した関数にムーブする
fn gives_ownership() -> String {
    let some_string = String::from("hello"); // some_stringがスコープに入る

    some_string // some_stringが返され、呼び出し元関数にムーブされる
}

// takes_and_gives_backは、Stringを1つ受け取り、返す
fn takes_and_gives_back(a_string: String) -> String { // a_stringがスコープに入る
    a_string // a_stringが返され、呼び出し元関数にムーブされる
}