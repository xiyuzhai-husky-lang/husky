use regex::Regex;
use std::fs;

//Checks if filepath points to a .toml file
fn is_toml_filepath(filepath: &str) -> bool {
    let toml_filepath_re = Regex::new(r"^.*\.toml$").unwrap();
    return toml_filepath_re.is_match(filepath);
}

//Takes a file path and reads its contents in as plain text
fn load_file_contents(filepath: &str) -> String {
    let file_contents =
        fs::read_to_string(filepath).expect("ERROR: Something went wrong reading the file");
    return file_contents;
}

fn load_toml_file(toml_filepath: &str) -> toml::Value {
    //Check if a valid .toml filepath
    if !is_toml_filepath(toml_filepath) {
        println!(
            "WARN: detected invalid path to .toml file:\n{}",
            toml_filepath
        )
    }
    //Fetch toml data
    let toml_raw = load_file_contents(toml_filepath);
    let toml_data = toml::de::from_str(&toml_raw);
    return toml_data.expect("ERROR: Failed to read improperly formatted TOML file!");
}

fn check_table_sorted(toml_table: &toml::value::Table) -> bool {
    let table_keys: Vec<String> = toml_table.keys().map(|key| key.to_owned()).collect();
    let mut sorted_table_keys = table_keys.clone();
    sorted_table_keys.sort_unstable();
    return table_keys == sorted_table_keys;
}

fn check_cargo_toml_sorted(toml_data: toml::Value) -> Option<String> {
    let included_headers: Vec<&str> = vec![
        "dependencies",
        "dev-dependencies",
        "build-dependencies",
        "workspace.members",
        "workspace.exclude",
    ];
    for table_header in included_headers.iter() {
        if toml_data.get(table_header).is_some() {
            let toml_table = toml_data.get(table_header).unwrap().as_table();
            if toml_table.is_some() {
                if !check_table_sorted(toml_table.unwrap()) {
                    return Some(table_header.to_string());
                }
            }
        }
    }
    return None;
}

//TODO: implement unit/integration tests for all major functions
//TODO: write functions to write a properly sorted Cargo.toml file to disk

fn organise_cargo_toml(toml_filepath: &str) {
    let toml_value = load_toml_file(toml_filepath);
    //Check if appropriate tables in file are sorted
    let toml_sort_result = check_cargo_toml_sorted(toml_value);
    if toml_sort_result.is_some() {
        eprintln!(
            "FAIL: found unsorted Cargo.toml table: {}",
            toml_sort_result.unwrap()
        );
        std::process::exit(65);
    }
    println!("PASS: the detected Cargo.toml file is properly sorted!");
    std::process::exit(0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_toml_filepath() {
        assert!(is_toml_filepath("/cargo.toml"));
        assert!(!is_toml_filepath("cargo.tomls"));
    }
}
