mod boxes;
mod deref;
mod refcell;
mod reference_counters;

fn main() {
    // Boxes allow for a single Owner, and immutable or mutable borrows, checked at compile time
    boxes::execute();
    deref::execute();

    // Rc's allow for multiple Owners of the same data, and only immutable borrows
    reference_counters::execute();

    // RefCells allow for a single Owner, and immutable or mutable borrows, checked at run time
    // we can mutate the value inside the RefCell, even when the RefCell itself is immutable: the interior mutability pattern
    refcell::execute();
}
