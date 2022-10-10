use colored::Colorize;
use husky_io_utils::diff_write;
use husky_path_utils::{Path, PathBuf};
use husky_print_utils::p;
use husky_print_utils::*;
use relative_path::RelativePathBuf;
use serde::{Deserialize, Serialize};
use std::{
    fs::read_to_string,
    io::{stdin, stdout, Write},
    ops::Deref,
};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
struct Expect<In, Out> {
    input: In,
    output: Out,
}

#[derive(PartialEq, Eq)]
pub enum Interactive {
    True,
    False,
}

#[macro_export]
macro_rules! expect_test {
    ($partial_paths: expr, $f: expr) => {
        use husky_test_utils::expect::*;

        fn run_tests(interactive: Interactive) {
            expect_test::<String, _>(interactive, $partial_paths, $f)
        }
        #[test]
        fn it_works() {
            run_tests(Interactive::False)
        }

        fn main() {
            run_tests(Interactive::True)
        }
    };
}

pub fn expect_test<Input, Output>(
    interative: Interactive,
    partial_path: &'static str,
    f: fn(&<Input as Deref>::Target) -> Output,
) where
    Input:
        for<'a> Deserialize<'a> + Serialize + std::fmt::Display + PartialEq + Eq + std::ops::Deref,
    Output: for<'a> Deserialize<'a> + Serialize + std::fmt::Display + PartialEq + Eq,
{
    let dir: PathBuf = std::env::var("CARGO_MANIFEST_DIR").unwrap().into();
    let input_path = dir.join(format!("tests/{partial_path}.json"));
    let inputs: Vec<Input> = deserialize_from_file(&input_path).unwrap();
    let expect_path = dir.join(format!("tests/{partial_path}.expect.json"));
    let outputs = inputs.iter().map(|input| f(input)).collect::<Vec<_>>();
    let expects = match deserialize_from_file(&expect_path) {
        Ok::<Vec<Expect<Input, Output>>, _>(expects) => expects,
        Err(e) => match e {
            DesIoError::IO(_) => todo!(),
            DesIoError::NotValidFile(_) | DesIoError::SerdeJson(_) => {
                vec![]
            }
        },
    };
    let updated_expects = if interative == Interactive::True {
        match ask_yes_or_no(format!(
            r#"{RED}Unable to parse json value from {expect_path:?}.{RESET}
Do you want to overwrite expect (y/n)? "#
        )) {
            true => {
                let mut updated_expects = vec![];
                for (input, output) in std::iter::zip(inputs.into_iter(), outputs.into_iter()) {
                    match ask_yes_or_no(format!(
                        r#"
{CYAN}[input]{RESET}
{}
{CYAN}[output]{RESET}
{}

is this okay (y/n)? "#,
                        textwrap::indent(&input.to_string(), "    ").blue(),
                        textwrap::indent(&output.to_string(), "    ").yellow(),
                    )) {
                        true => updated_expects.push(Expect { input, output }),
                        false => {
                            diff_write(
                                &expect_path,
                                &serde_json::to_string_pretty(&updated_expects).unwrap(),
                                true,
                            );
                            panic!("expect update not accepted")
                        }
                    }
                }
                updated_expects
            }
            false => {
                panic!("Unable to parse json value from {expect_path:?}")
            }
        }
    } else {
        panic!("Unable to parse json value from {expect_path:?}")
    };
    diff_write(
        &expect_path,
        &serde_json::to_string_pretty(&updated_expects).unwrap(),
        true,
    );
}

#[must_use]
enum OverwritePermission {
    Allowed,
    Forbidden,
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
    let value = serde_json::from_str(&content)?;
    Ok(serde_json::from_value(value)?)
}
