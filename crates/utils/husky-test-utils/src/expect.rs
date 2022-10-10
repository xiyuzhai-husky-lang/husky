use husky_io_utils::diff_write;
use husky_path_utils::PathBuf;
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
    let input_path = &format!("tests/{partial_path}.in.json").into();
    let inputs: Vec<Input> = deserialize_from_file(&input_path).unwrap();
    let expect_path = format!("tests/{partial_path}.expect.json").into();
    let outputs = inputs.iter().map(|input| f(input)).collect::<Vec<_>>();
    let updated_expects = match deserialize_from_file(&expect_path) {
        Ok::<Vec<Expect<Input, Output>>, _>(mut expects) => {
            assert_eq!(outputs.len(), expects.len());
            for (output, expect) in std::iter::zip(outputs.into_iter(), expects.iter_mut()) {
                if output != expect.output {
                    todo!()
                }
            }
            expects
        }
        Err(e) => match e {
            DesIoError::IO(_) => todo!(),
            DesIoError::SerdeJson(_) => {
                if interative == Interactive::True {
                    match ask_yes_or_no(format!(
                        r#"Unable to parse json value from {expect_path:?}.
Do you want to overwrite expect (y/n)? "#
                    )) {
                        true => {
                            let mut updated_expects = vec![];
                            for (input, output) in
                                std::iter::zip(inputs.into_iter(), outputs.into_iter())
                            {
                                match ask_yes_or_no(format!(
                                    r#"
input:

{input}

output:

{output}

is this okay (y/n)? "#
                                )) {
                                    true => updated_expects.push(Expect { input, output }),
                                    false => {
                                        diff_write(
                                            &expect_path.to_path("."),
                                            &serde_json::to_string(&updated_expects).unwrap(),
                                            true,
                                        );
                                        panic!()
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
                }
            }
        },
    };
    diff_write(
        &expect_path.to_path("."),
        &serde_json::to_string(&updated_expects).unwrap(),
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
    #[error("io")]
    IO(#[from] std::io::Error),
    #[error("serde json")]
    SerdeJson(#[from] serde_json::error::Error),
}

fn deserialize_from_file<T>(rel_path: &RelativePathBuf) -> Result<T, DesIoError>
where
    T: for<'a> Deserialize<'a>,
{
    // let mut cargo_manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let path = rel_path.to_path(".");
    if !path.exists() || !path.is_file() {
        println!("{:?}", PathBuf::from(".").canonicalize());
        panic!("path = {path:?}")
    }
    let content = read_to_string(&path)?;
    let value = serde_json::from_str(&content)?;
    Ok(serde_json::from_value(value)?)
}
