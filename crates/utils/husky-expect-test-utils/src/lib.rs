mod config;

use colored::Colorize;
use config::*;
use husky_io_utils::diff_write;

use husky_print_utils::*;
use serde::{Deserialize, Serialize};
use std::{
    fs::read_to_string,
    io::{stdin, stdout, Write},
    ops::Deref,
    path::{Path, PathBuf},
};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
struct Expect<Input, Output> {
    input: Input,
    #[serde(skip_serializing_if = "Option::is_none")]
    output: Option<Output>,
}

pub fn expect_test<Input, Output>(
    relative_folder_path: &str,
    f: &impl Fn(&<Input as Deref>::Target) -> Output,
) where
    Input: for<'a> Deserialize<'a>
        + Serialize
        + std::fmt::Display
        + std::fmt::Debug
        + PartialEq
        + Eq
        + std::ops::Deref,
    Output:
        for<'a> Deserialize<'a> + Serialize + std::fmt::Display + std::fmt::Debug + PartialEq + Eq,
{
    let flag = ExpectTestConfig::from_env();
    let dir: PathBuf = std::env::var("CARGO_MANIFEST_DIR").unwrap().into();
    let tests_dir = dir.join("tests").join(relative_folder_path);
    assert!(tests_dir.exists());
    assert!(tests_dir.is_dir());
    let mut test_paths: Vec<PathBuf> = vec![];
    for entry in std::fs::read_dir(tests_dir).unwrap() {
        let entry = entry.unwrap();
        let subpath = entry.path();
        if subpath.is_dir() {
            todo!()
        } else {
            let file_stem = subpath.file_name().unwrap().to_str().unwrap();
            let splits: Vec<_> = file_stem.split(".").collect();
            assert_eq!(splits[1], "test");
            // ad hoc
            if splits[2] == "md" {
                continue;
            }
            assert_eq!(splits[2], "json");
            assert_eq!(splits.len(), 3);
            test_paths.push(subpath)
        }
    }

    for test_path in test_paths {
        let mut expects = match deserialize_from_file(&test_path) {
            Ok::<Vec<Expect<Input, Output>>, _>(expects) => expects,
            Err(e) => panic!("unable to parse results because due to {e} for {test_path:?}"),
        };
        if flag == ExpectTestConfig::UpdateExpect {
            for expect in expects.iter_mut() {
                let output = f(&expect.input);
                if expect.output.as_ref() != Some(&output) {
                    match ask_is_input_output_okay(&expect.input, &output) {
                        true => expect.output = Some(output),
                        false => {
                            diff_write(
                                &test_path,
                                &serde_json::to_string_pretty(&expects).unwrap(),
                                true,
                            );
                            panic!("{RED}expect update not accepted{RESET}")
                        }
                    }
                }
            }
            diff_write(
                &test_path,
                &serde_json::to_string_pretty(&expects).unwrap(),
                true,
            );
        } else {
            for expect in expects.iter() {
                let output = f(&expect.input);
                assert_eq!(expect.output, Some(output))
            }
        }
    }
}

fn ask_is_input_output_okay<Input, Output>(input: &Input, output: &Output) -> bool
where
    Input: for<'a> Deserialize<'a>
        + Serialize
        + std::fmt::Display
        + std::fmt::Debug
        + PartialEq
        + Eq
        + std::ops::Deref,
    Output:
        for<'a> Deserialize<'a> + Serialize + std::fmt::Display + std::fmt::Debug + PartialEq + Eq,
{
    ask_yes_or_no(format!(
        r#"
{CYAN}[input]{RESET}
{}
{CYAN}[output]{RESET}
{}

is this okay (y/n)? "#,
        textwrap::indent(&input.to_string(), "    ").blue(),
        textwrap::indent(&output.to_string(), "    ").yellow(),
    ))
}

fn ask_yes_or_no(message: String) -> bool {
    loop {
        print!("{message}");
        let _ = stdout().flush();
        let mut s = String::new();
        stdin()
            .read_line(&mut s)
            .expect("Did not enter a correct string");
        if let Some('\n') = s.chars().next_back() {
            s.pop();
        }
        if let Some('\r') = s.chars().next_back() {
            s.pop();
        }
        match &s as &str {
            "y" => break true,
            "n" => break false,
            _ => println!("Invalid answer: {}", s),
        }
    }
}

use thiserror::Error;

#[derive(Error, Debug)]
enum DesIoError {
    #[error("not valid file")]
    NotValidFile(PathBuf),
    #[error("io")]
    IO(#[from] std::io::Error),
    #[error("serde json")]
    SerdeJson(#[from] serde_json::error::Error),
}

fn deserialize_from_file<T>(path: &Path) -> Result<T, DesIoError>
where
    T: for<'a> Deserialize<'a>,
{
    if !path.exists() || !path.is_file() {
        return Err(DesIoError::NotValidFile(path.to_owned()));
    }
    let content = read_to_string(&path)?;
    Ok(serde_json::from_str(&content)?)
}
