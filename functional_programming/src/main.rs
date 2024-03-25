mod closures;
use closures::{Inventory, ShirtColor};
mod iterators;

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    let example_closure = |x| x;

    let _s = example_closure(String::from("hello"));

    // let n = example_closure(5);
    // we can't do this now, since the earlier call cemented the types the closure accepts and returns

    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let mut borrows_mutably = || list.push(7);

    // println!("{:?}", list);
    // we cannot borrow `list` again, since Rust can see in the closure that we've already mutably borrowed it (to push a new value on), and we can't have another borrow while that one is possible

    borrows_mutably();
    println!("After calling closure: {:?}", list);

    iterators::get_iterator();
    iterators::mapping();
}
