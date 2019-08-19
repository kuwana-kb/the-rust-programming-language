fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let i32_list = vec![10, 20, 55, 100, 43];
    let result = largest(&i32_list);
    println!("{}", result);

    let char_list = vec!['y', 'b', 'u', 's'];
    let result = largest(&char_list);
    println!("{}", result);
}