use husky_cybertron_mini_husky_compiler::rnd::basic::rnd_codes;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    let dir = PathBuf::from("data/mini-husky/basic");
    assert!(dir.exists());

    let n = 10; // Example value for n
    let dataset_filename = dir.join(format!("dataset-{}.txt", n));

    // Generate the random codes
    let data = rnd_codes(n);

    // Write to the file
    let mut file = fs::File::create(&dataset_filename).expect("Unable to create file");
    file.write_all(data.as_bytes())
        .expect("Unable to write data");

    println!("Data written to {:?}", dataset_filename);
}
