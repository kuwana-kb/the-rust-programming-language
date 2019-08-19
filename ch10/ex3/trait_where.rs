pub fn notify<T: Summary>(item: T) {
    println!("Breaking new! {}", item.summerize());
}

fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {

}

fn some_function2<T, U>(t: T, u: U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{
}