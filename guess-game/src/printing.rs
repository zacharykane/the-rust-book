#[derive(Debug)]
struct DebugPrintable(i32);

fn private_print() {
    println!("called private function");

    let up = DebugPrintable(24);
    println!("{:?}", up)
}

pub fn public_print() {
    private_print()
}
