use subprocess::SUBPROCESS;

mod subprocess;

pub fn add(left: u64, right: u64) -> u64 {
    // Example: Send numbers to container program for addition
    let numbers = vec!["5", "3"];

    let mut subprocess = SUBPROCESS.lock().unwrap();
    subprocess.write_line(numbers.join(" "));

    // Read result from container's stdout
    let line = subprocess.read_line().unwrap();
    // Only print lines that contain numeric results
    line.trim().parse().unwrap()
}

#[test]
fn add_works() {
    assert_eq!(add(5, 3), 8);
}
