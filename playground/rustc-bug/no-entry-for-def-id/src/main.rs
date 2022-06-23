mod haha;

use haha::*;
pub static A3: &A = &A { items: &[&A1, &A2] };

fn main() {
    println!("Hello, world!");
}
