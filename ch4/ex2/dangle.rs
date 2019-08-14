fn main() {
    let reference_to_nothing = dangle();
}

// dangleはStringへの参照を返す
fn dangle() -> &String {
    let s = String::from("hello"); // sは新しいString

    &s // error: missing lifetime specifier
} // ここでsはスコープを抜け、ドロップされる。そのメモリは消されるので危険