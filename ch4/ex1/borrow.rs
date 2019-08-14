fn main() {
    let s = String::from("hello"); // sがスコープに入る

    takes_ownership(s); // sの値が関数にムーブされ
    // ここではもう有効ではない

    // println!("{}", s); // error borrow of moved value

    let x = 5; // xがスコープに入る

    makes_copy(x); //xも関数にムーブされるが

    println!("{}", x); // i32はCopyなので、ここでxを使っても大丈夫
}

fn takes_ownership(some_string: String) { //some_stringがスコープに入る
    println!("{}", some_string);
} // ここでsome_stringがスコープを抜け、`drop`が呼ばれる。後ろ盾していたメモリが開放される

fn makes_copy(some_integer: i32) { // some_integerがスコープに入る
    println!("{}", some_integer);
} // ここでsome_integerがスコープを抜ける