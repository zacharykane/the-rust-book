pub fn run() {
    println!("=== Vector ===\n");

    let v = vec![1, 2, 3];
    // note: a reference & is the same as calling .iter() on a variable
    for i in &v {
        println!("{i}");
    }
    println!("{:?}", v);

    // holding different types in one vector with Enums
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("{:?}", row);

    println!("\n=== Vector ===\n\n");
}
