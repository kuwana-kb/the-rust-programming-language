fn main() {
    let v = vec![100, 32, 57];

    println!("{:p}", &v);

    for i in &v {
        println!("{}", i);
    }

    println!("{:?}",v);

    let mut v = vec![100, 32, 57];

    for i in &mut v {
        *i += 50;
    }
    println!("{:?}", v);


    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("blue")),
    ];
    println!("{:?}", row);
}

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

