use std::collections::HashMap;

pub fn run() {
    println!("=== HashMap ===\n");

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 20);
    println!("{:?}", scores);

    let a_score = scores.get("Red").copied().unwrap_or(0);
    println!("{:?}", a_score);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // update, overwriting whatever value was set for Red before
    scores.insert(String::from("Red"), 50);
    // update, overwrite the value if it does not exist for Red before
    scores.entry(String::from("Red")).or_insert(0);

    // count the number of occurances of each word in the string literal
    let some_text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in some_text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        // dereference, because you cannot add an integer value to a reference
        *count += 1;
    }
    println!("{:?}", map);

    println!("{:?}", scores);
    println!("\n=== HashMap ===\n\n");
}
