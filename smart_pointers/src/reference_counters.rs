use std::rc::Rc;

// List now stores on the Heap.
// the Cons variant only needs enough memory to store a i32 and a pointer
pub enum RcList {
    Cons(i32, Rc<RcList>),
    Nil,
}

impl RcList {
    pub fn recurse(&self) {
        match self {
            RcList::Cons(val, next) => {
                println!("value: {val}");
                // Rc's are like references, pointing to their data on the Heap
                RcList::recurse(next)
            }
            RcList::Nil => println!("Done."),
        }
    }
}

pub fn execute() {
    let a = Rc::new(RcList::Cons(
        5,
        Rc::new(RcList::Cons(10, Rc::new(RcList::Nil))),
    ));
    println!("count after creating a = {}", Rc::strong_count(&a));
    // We could have called a.clone() rather than Rc::clone(&a), but Rust’s convention is to use Rc::clone in this case.

    // The implementation of Rc::clone doesn’t make a deep copy of all the data like most types’ implementations of clone do.

    //The call to Rc::clone only increments the reference count, which doesn’t take much time.
    let b = RcList::Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));

    {
        let c = RcList::Cons(5, Rc::clone(&a));
        c.recurse();
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after dropping c = {}", Rc::strong_count(&a));

    a.recurse();
    b.recurse();
}
