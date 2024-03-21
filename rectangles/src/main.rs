// Structs: represent arbitrary data
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // borrow a reference
    fn calculate_area(self: &Self) -> u32 {
        self.width * self.height
    }
    // borrow a mutable reference
    fn update(self: &mut Self) -> &Self {
        self.width = 100;
        self
    }
    // take ownership
    fn change(self: Self) -> Different {
        println!("{:?}", self);
        Different
    }
    // without a first Self parameter, associated function is callable from the type namespace
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

// unit-like struct
#[derive(Debug)]
struct Different;

// Tuple struct
#[derive(Debug)]
struct Point(i32, i32);

// Enums: group arbitrary representations
#[derive(Debug)]
enum Choice {
    TypeOne,
    TypeTwo,
}

#[derive(Debug)]
enum Variant {
    One(String),
    Two(bool),
    Three { this: String, that: f32 },
}

impl Variant {
    fn call(&self) {
        match &self {
            Variant::One(a) => println!("a One, with '{}'", a),
            Variant::Two(b) => println!("a Two, with: {}", b),
            Variant::Three { this, that } => println!("a Three, with {} and {}", this, that),
        }
    }
}

fn main() {
    let scale = 2;
    let rect = Rectangle {
        width: dbg!(50 * scale),
        height: 30,
    };

    dbg!(&rect);
    println!("the area of {:#?} is {}", rect, rect.calculate_area());

    let mut rect2 = Rectangle {
        width: 10,
        height: 10,
    };

    println!("update: {:?}", rect2.update());
    println!("change: {:?}", rect2.change());

    let square = Rectangle::square(5);
    println!("a square Rectacngle: {:?}", square);

    let p1 = Point(3, 4);
    println!("x of Point p1 is: {:?}", p1.0);

    let a_choice = Choice::TypeOne;
    let b_choice = Choice::TypeTwo;
    accept(a_choice);
    accept(b_choice);

    let var1 = Variant::One(String::from("Some value"));
    let var2 = Variant::Two(true);
    let var3 = Variant::Three {
        this: String::from("this value"),
        that: 123.456,
    };
    var1.call();
    var2.call();
    var3.call();

    // do something if var3 is this Variant
    if let Variant::Three { this, that } = var3 {
        println!("and a Three with: {}, {}", this, that);
    }
}

// can take any variant of the Choice enum
fn accept(some_choice: Choice) {
    println!("{:?}", some_choice);
    if let Choice::TypeOne = some_choice {
        println!("^ whoa we got a One!");
    }
}
