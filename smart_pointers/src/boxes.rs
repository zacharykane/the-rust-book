// List now stores on the Heap.
// the Cons variant only needs enough memory to store a i32 and a pointer
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

impl List {
    pub fn recurse(l: List) {
        match l {
            List::Cons(val, next) => {
                println!("value: {val}");
                // Boxes are like references, pointing to their data on the Heap
                List::recurse(*next)
            }
            List::Nil => println!("Done."),
        }
    }
}

pub fn execute() {
    // Boxes move storage to the Heap, rather than the Stack
    let b = Box::new(5);
    println!("b = {}", b);

    let list = List::Cons(
        1,
        Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
    );

    List::recurse(list);

    // Boxes behave like normal references
    // use deref operator to access value "inside" or "pointed to"
    let x = 5;
    let y = &x;
    let z = Box::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);
    assert_eq!(5, *z);
}
