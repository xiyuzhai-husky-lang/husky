use husky_cybertron_mini_husky_compiler::rnd::basic::rnd_codes;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    let dir = PathBuf::from("data/mini-husky/basic");
    assert!(dir.exists());

    // Predefined sets of parameters
    let params = vec![(10000, 10, 0.3), (50000, 15, 0.2), (100000, 20, 0.1)];

    // Keep track of files we're going to write
    let mut files_to_keep = Vec::new();

    for (n, max_fns, error_rate) in &params {
        let dataset_filename = dir.join(format!(
            "dataset-n{}-f{}-e{:.2}.txt",
            n, max_fns, error_rate
        ));
        files_to_keep.push(dataset_filename.clone());

        // Generate the random codes
        let data = rnd_codes(*n, *max_fns, *error_rate);

        // Write to the file
        let mut file = fs::File::create(&dataset_filename).expect("Unable to create file");
        file.write_all(data.as_bytes())
            .expect("Unable to write data");

        println!("Data written to {:?}", dataset_filename);
    }

    // Clear other files in the folder
    for entry in fs::read_dir(&dir).expect("Unable to read directory") {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.is_file() && !files_to_keep.contains(&path) {
                fs::remove_file(&path).expect("Unable to remove file");
                println!("Removed old file: {:?}", path);
            }
        }
    }
}
