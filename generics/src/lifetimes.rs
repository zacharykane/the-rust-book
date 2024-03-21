use std::fmt::Display;

/**
 * a public function that:
 * - declares lifetimes since we return a reference related to multiple parameters, potentially
 * - accepts a generic type T
 * - that generic type must implement the Display Trait
 */
pub fn longest<'a, T>(x: &'a str, y: &'a str, announcement: T) -> &'a str
where
    T: Display,
{
    println!("{}", announcement);

    if x.len() > y.len() {
        x
    } else {
        y
    }
}

pub struct ImportantExcerpt<'a> {
    pub part: &'a str,
}
