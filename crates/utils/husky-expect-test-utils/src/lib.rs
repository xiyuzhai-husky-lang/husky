mod ask;
mod config;
mod entry;
mod path;

use ask::*;
use colored::Colorize;
use config::*;
use entry::*;
use husky_io_utils::diff_write;
use husky_print_utils::*;
use path::*;
use serde::{Deserialize, Serialize};
use std::{
    fs::read_to_string,
    io::{stdin, stdout, Write},
    ops::Deref,
    path::{Path, PathBuf},
};

pub fn expect_test_husky_to_rust(relative_test_dir: &str, f: &impl Fn(&str) -> String) {
    let config = ExpectTestConfig::from_env();
    for test_path in collect_test_paths(relative_test_dir) {
        let mut expects = match Entry::extract_from_file(&test_path) {
            Ok::<Vec<Entry>, _>(expects) => expects,
            Err(e) => panic!("unable to parse results because due to {e} for {test_path:?}"),
        };
        if config == ExpectTestConfig::UpdateExpect {
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
