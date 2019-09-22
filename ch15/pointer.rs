fn main() {
    let x = 5;
    // let y = &x;
    // let y = Box::new(x);
    let y = MyBox::new(x);


    assert_eq!(x, 5);
    assert_eq!(*y, 5);
}


struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(v: T) -> MyBox<T> {
        MyBox(v)
    }
}
