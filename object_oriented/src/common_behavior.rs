pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    /**
     * have our GUI library contain a collection of trait object implementors
     *
     * defining a Trait Object:
     * - a pointer (ref or smart pointer),
     * - `dyn`,
     * - the Trait
     *
     * the trait object points to:
     * - an instance of a type implementing the trait
     * - a table to lookup trait methods on that type, at runtime
     */
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// alternative implementation using generics with trait bounds
pub struct ScreenTwo<T: Draw> {
    pub components: Vec<T>,
}

impl<T> ScreenTwo<T>
where
    T: Draw,
{
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!(
            "this would draw Button to screen of size: {} {} and text: {}",
            self.width, self.height, self.label
        )
    }
}
