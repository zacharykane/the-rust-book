pub fn run() {
    println!("=== String ===\n");

    let mut s1 = String::new();
    s1.push_str("hey");
    println!("{s1}");

    let s2 = "huh?";
    // equivalent to s2.to_string()
    let mut s3 = String::from(s2);
    // push a String or &str
    s3.push_str(" hey");
    // push a char
    s3.push('X');
    println!("{s3} {s2}");

    println!("\n=== String ===\n\n");
}
