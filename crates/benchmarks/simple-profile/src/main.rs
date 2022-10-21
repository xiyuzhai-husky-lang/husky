use std::time::Instant;

fn main() {
    let now = Instant::now();
    let mut x = 1.1f64;
    for _ in 0..400000000 {
        x += (x).ln() - (x - 1.)
    }
    println!("{} seconds elapsed for x = {x}", now.elapsed().as_secs())
}
