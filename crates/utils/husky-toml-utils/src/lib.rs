use serde::Deserialize;
use std::fs;
use std::path::Path;

// Function to read TOML file and deserialize it into type T
pub fn read_toml<T: for<'a> Deserialize<'a>>(path: &Path) -> Result<T, Box<dyn std::error::Error>> {
    // Read the file to a string
    let contents = fs::read_to_string(path)?;

    // Deserialize the string into type T
    let deserialized = toml::from_str(&contents)?;

    Ok(deserialized)
}
