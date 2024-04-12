mod encapsulation;
use encapsulation::AveragedCollection;
mod common_behavior;
use common_behavior::Draw;

use crate::common_behavior::{Button, Screen, ScreenTwo};

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!(
            "this would draw Button to screen of size: {} {} and list of options: {:?}",
            self.width, self.height, self.options
        );
    }
}

fn main() {
    println!("Hello, world!");

    let mut al = AveragedCollection::new();

    al.add(32);
    al.add(16);
    al.add(1);

    println!("{}", al.average());

    al.remove();

    println!("{}", al.average());

    let renderer = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 44,
                options: vec![
                    String::from("Yes"),
                    String::from("No"),
                    String::from("Maybe..."),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 44,
                label: String::from("Submit"),
            }),
        ],
    };

    renderer.run();

    // can't work with trait bounds since the first concrete type supplied to our Vector constrains the following types to itself
    let renderer_two = ScreenTwo {
        components: vec![
            SelectBox {
                width: 75,
                height: 44,
                options: vec![
                    String::from("Yes"),
                    String::from("No"),
                    String::from("Maybe..."),
                ],
            },
            Button {
                width: 50,
                height: 44,
                label: String::from("Submit"),
            },
        ],
    };
}
