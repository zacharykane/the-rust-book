mod generics;
mod lifetimes;
mod traits;

use crate::generics::Point;
use crate::lifetimes::ImportantExcerpt;
use crate::traits::{BlogPost, Summary, Toot};

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = generics::largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'a', 'o', 'q', 'z'];

    let result = generics::largest(&char_list);
    println!("The largest number is {}", result);

    let int_point = Point { x: 2, y: 5 };
    let float_point = Point { x: 3.3, y: 9.2 };
    let mixed_point = Point { x: 10, y: 1.234 };

    println!("{}", int_point.x());
    println!("{}", float_point.x());
    println!("{}", mixed_point.x());

    println!("{}", float_point.distance_from_origin());

    let toot = Toot {
        content: String::from(
            "Highly recommend Matt Jencik and Tapes & Topographies, but I digress",
        ),
        username: String::from("zach"),
        is_reply: false,
        is_retoot: false,
    };

    println!("a new toot: {}", toot.summarize());

    let post = BlogPost {
        content: String::from("Even more games of the year..."),
        author: String::from("Mu'ad Diib"),
        web_ring: String::from("Fremen"),
    };

    traits::notify(&post);

    let string1 = String::from("abcd");
    let result;

    // this won't compile, since the value of `string2` will be dropped at the end of this nested scope. we send a reference (.as_str() call) into longest that _could_ be returned and set to `result`
    // if we tried to use `result` outside of this inner scope, it could reference `string2` which no longer exists. lifetimes prevent us from doing this (creating a dangling reference)
    {
        let string2 = String::from("xyz");
        result = lifetimes::longest(
            string1.as_str(),
            string2.as_str(),
            "Hey we're learning about Generics, Trait Bounds, & Lifetimes.",
        );
    }

    println!("the longest string is {}", result);

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("did not find a '.'");
    let ie = ImportantExcerpt {
        part: first_sentence,
    };
    println!("first sentence of excerpt is: {}", ie.part);
}
