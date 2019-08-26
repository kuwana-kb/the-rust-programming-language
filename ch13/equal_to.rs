fn main() {
    let x = 4;
    let equal_to = |z| z == x;
    let y = 4;

    println!("{:?}", assert!(equal_to(y)));
}