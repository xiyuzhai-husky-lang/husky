use husky_cybertron_mini_husky_compiler::rnd::basic::{rnd_codes, TokenInfo};
use std::fs;
use std::io::{self, Read, Write};
use std::path::Path;
use std::path::PathBuf;

fn preview_data_strings(data: &Vec<(Vec<String>, Vec<TokenInfo>)>) {
    'outer: for (strings, _tokens) in data {
        for s in strings {
            print!("{} ", s);
        }
        break 'outer;
    }
    println!();
}

fn main() {
    let dir = PathBuf::from("data/mini-husky/basic");
    assert!(dir.exists());

    // Predefined sets of parameters
    let params = vec![
        (10000, 10, 0.5),
        (50000, 15, 0.5),
        (100000, 20, 0.5),
        (1000000, 20, 0.5),
    ];

    // Keep track of files we're going to write
    let mut files_to_keep = Vec::new();

    for (n, max_fns, error_rate) in &params {
        let dataset_filename = dir.join(format!(
            "dataset-n{}-f{}-e{:.2}.msgpack",
            n, max_fns, error_rate
        ));
        files_to_keep.push(dataset_filename.clone());

        // Generate the random codes
        let data = rnd_codes(*n, *max_fns, *error_rate);

        preview_data_strings(&data);

        // Write to the file
        let mut file = fs::File::create(&dataset_filename).expect("Unable to create file");
        // file.write_all(&rmp_serde::to_vec(&data).expect("Unable to serialize data"))
        //     .expect("Unable to write data
        write_if_different(&dataset_filename, &data).expect("Unable to write data");
    }

    // Clear other files in the folder
    for entry in fs::read_dir(&dir).expect("Unable to read directory") {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.is_file()
                && path.extension().unwrap_or_default() == "msgpack"
                && !files_to_keep.contains(&path)
            {
                fs::remove_file(&path).expect("Unable to remove file");
                println!("Removed old file: {:?}", path);
            }
        }
    }
}

fn write_if_different<T: serde::Serialize>(dataset_filepath: &Path, data: &T) -> io::Result<()> {
    let new_data = rmp_serde::to_vec(&data).expect("Unable to serialize data");

    if dataset_filepath.exists() {
        let mut existing_file = fs::File::open(&dataset_filepath)?;
        let mut existing_data = Vec::new();
        existing_file.read_to_end(&mut existing_data)?;

        // Compare the current and new data
        if existing_data == new_data {
            println!("No changes detected, skipping write.");
            return Ok(()); // Skip writing if the data is identical
        }
    }

    let mut file = fs::File::create(&dataset_filepath)?;
    file.write_all(&new_data)?;
    println!("Data written to {:?}", dataset_filepath);

    Ok(())
}
