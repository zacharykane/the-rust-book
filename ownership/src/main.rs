#[derive(Debug)]
struct WillDrop;

impl Drop for WillDrop {
    fn drop(&mut self) {
        println!("    drop destructor called! WillDrop is being dropped, aka freed");
    }
}

#[derive(Debug)]
struct User {
    active: bool,
    name: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    // 1. Ownership of Heap data
    //
    let s = String::from("hi hi");

    take_ownership(s);

    let x = 5;

    make_copy(x);

    // println!("{} {}", x, s);
    // cannot do like this, since s will be owned by the scope in the take_ownership function. when that finishes, s will be dropped from memory

    // 2. Moving who Owns Heap data
    //
    let u = gives_ownership();
    // u moved into v, u is dropped
    let v = takes_and_gives_ownership(u);
    println!("{v}");
    // v moved into owning_calculate_length, v is dropped
    let (v2, len) = owning_calculate_length(v);
    println!("the length of \"{v2}\" is {len}");

    // we can't refer to v anymore since it moved to v2
    // how can we just get a length calculation and keep our String?

    // 3. using References to Borrow Heap data
    //
    let mut v3 = String::from("Hello world");
    let len2 = reference_borrowing_calculate_length(&mut v3);
    println!("the length of '{}' is '{}", v3, len2);

    // 4. using slices to refer to pieces of Heap data
    //
    //    String references, whole string slices and string literals
    //    are equivalent
    //    &String == &str == "the type of this literal"
    let w = String::from("Shiba Inus are funny");
    let shiba = &w[0..5];
    let inu = &w[5..10];
    println!("{}{}", shiba, inu);

    let first = first_word(&w);
    let first_from_whole_slice = first_word(&w[..]);
    let first_from_half_slice = first_word(&w[6..]);

    println!("the first word is: '{}'", first);
    println!("the first word is: '{}'", first_from_whole_slice);
    println!("the first word is: '{}'", first_from_half_slice);

    // 5. slices for Arrays work similarly
    //    the range defines the start index, out to a length - 1
    let arr = [1, 2, 3, 4, 5];
    let arr_slice = &arr[1..3];
    println!("{:?} {:?}", arr, arr_slice);

    // Deconstructor: the drop implementation is ran when this owning scope ends
    let wd = WillDrop;
    println!("made a {:?}", wd);

    let user1 = User {
        name: String::from("Zach"),
        email: String::from("zach@zach.com"),
        active: true,
        sign_in_count: 0,
    };
    let dupe = User {
        email: String::from("other@zach.com"),
        ..user1
    };
    // note that Struct fields that don't implement Copy, which makes creating new structs based on others follow the same borrowing rules as simpler variables: user1 as a whole is invalid now, except for the fields that can Copy since they moved into dupe
    println!("dupe based on user1: {:?}", dupe);
    println!("user1 is active? {:?}", user1.active);
}

fn make_copy(some_number: i32) {
    println!("{}", some_number);
}

fn take_ownership(some_string: String) {
    // variable taken in from argument
    // variable goes out of scope here when function ends
    println!("{}", some_string);
    // caller no longer has access to given String
}

fn gives_ownership() -> String {
    // variable created in scope
    let some_string = String::from("Hey there");
    // variable returned, moving ownership to calling scope
    some_string
}

fn takes_and_gives_ownership(some_string: String) -> String {
    some_string
}

fn owning_calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    println!("{}", length);

    (s, length)
}

fn reference_borrowing_calculate_length(some_string: &mut String) -> usize {
    some_string.push_str(", how are you?");
    some_string.len()
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (index, &segment) in bytes.iter().enumerate() {
        if segment == b' ' {
            return &s[0..index];
        }
    }

    &s[..]
}
