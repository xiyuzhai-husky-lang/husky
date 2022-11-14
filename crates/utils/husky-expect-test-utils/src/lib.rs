mod ask;
mod config;
mod content;
mod path;

use ask::*;
use colored::Colorize;
use config::*;
use content::{ExpectContent, UpdateExpectError};
use husky_io_utils::diff_write;
use husky_print_utils::*;
use path::*;
use serde::{Deserialize, Serialize};
use std::{
    fs::read_to_string,
    io::{stdin, stdout, Write},
    ops::Deref,
    panic::{RefUnwindSafe, UnwindSafe},
    path::{Path, PathBuf},
    process::exit,
};

pub fn expect_test_husky_to_rust(
    relative_test_dir: &str,
    f: &(impl Fn(&str) -> String + UnwindSafe + RefUnwindSafe),
) {
    let config = ExpectTestConfig::from_env();
    for test_path in collect_test_paths(relative_test_dir) {
        let mut expects = ExpectContent::extract_from_file(test_path)
            .unwrap_or_else(|e| panic!("unable to parse results because due to {e}"));
        if config == ExpectTestConfig::UpdateExpect {
            match expects.update(f) {
                Ok(_) => (),
                Err(e) => match e {
                    UpdateExpectError::OutputNotAccepted => panic!("output not accepted"),
                    UpdateExpectError::DerivedPanic(_) => exit(1),
                },
            }
        } else {
            expects.verify(f)
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
