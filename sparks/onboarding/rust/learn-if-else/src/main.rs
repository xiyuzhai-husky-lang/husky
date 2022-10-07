fn f1() {
    let x = 1;
    if x > 0 {
        println!("x is positive")
    } else {
        println!("x is not positive")
    }
}

// boolean = true | false
fn is_number_positive(x: i32) -> bool {
    if x > 0 {
        true
    } else {
        false
    }
}

fn main() {
    println!("is -1 positive? {}", is_number_positive(-1))
}
