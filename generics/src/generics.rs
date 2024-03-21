/**
 * largets can now accept a vector reference containing any type
 * with the constraint that the type T, must implement the Trait: PartialOrd
 * which allows objects to be compared (<, >, ==, etc)
 */
pub fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// we can have multiple generics to mix types in a Struct or Enum
#[derive(Debug)]
pub struct Point<T, U> {
    pub x: T,
    pub y: U,
}
impl<T, U> Point<T, U> {
    pub fn x(&self) -> &T {
        &self.x
    }
}
// applies only for concrete types of floats
impl Point<f32, f32> {
    pub fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
