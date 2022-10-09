use husky_path_utils::PathBuf;
use relative_path::RelativePathBuf;
use serde::{Deserialize, Serialize};
use std::{
    fs::read_to_string,
    io::{stdin, stdout, Write},
    ops::Deref,
};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
struct Entry<In, Out> {
    input: In,
    output: Out,
}

pub fn expect_test<In, Out>(partial_path: &'static str, f: fn(&<In as Deref>::Target) -> Out)
where
    In: for<'a> Deserialize<'a> + Serialize + PartialEq + Eq + std::ops::Deref,
    Out: for<'a> Deserialize<'a> + Serialize + PartialEq + Eq,
{
    let in_path = &format!("tests/{partial_path}.in.json").into();
    let in_content: Vec<In> = deserialize_from_file(&in_path).unwrap();
    let out_path = format!("tests/{partial_path}.out.json").into();
    match deserialize_from_file(&out_path) {
        Ok::<Vec<Entry<In, Out>>, _>(out_content) => {
            let out = in_content
                .into_iter()
                .map(|input| Entry {
                    output: f(&input),
                    input,
                })
                .collect::<Vec<_>>();
            if out != out_content {
                todo!()
            }
        }
        Err(e) => match e {
            DesIoError::IO(_) => todo!(),
            DesIoError::SerdeJson(_) => {
                ask_for_overwrite(format!("unable to parse json value from {out_path:?}"));
                return;
            }
        },
    }
}

#[must_use]
enum OverwritePermission {
    Allowed,
    Forbidden,
}

fn ask_for_overwrite(message: String) -> OverwritePermission {
    loop {
        print!("{message}. Do you want to overwrite expect (y/n)? ");
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
            "y" => break OverwritePermission::Allowed,
            "n" => break OverwritePermission::Forbidden,
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
