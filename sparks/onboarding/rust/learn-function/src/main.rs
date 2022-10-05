// fn short for "function"
fn sum(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let s = sum(1, 2);
    println!("s = {}, {}", s, s + 1);
}
