mod ask;
mod config;
mod instance;
mod path;

use ask::*;
use colored::Colorize;
use config::*;
use husky_io_utils::diff_write;
use husky_print_utils::*;
use instance::{ExpectInstance, UpdateExpectError};
use path::*;
use serde::{Deserialize, Serialize};
use std::{
    fs::read_to_string,
    io::{stdin, stdout, Write},
    panic::{RefUnwindSafe, UnwindSafe},
    path::{Path, PathBuf},
    process::exit,
};

pub fn expect_test_snippets(
    relative_test_dir: &str,
    f: &(impl Fn(&str) -> String + UnwindSafe + RefUnwindSafe),
) {
    let config = ExpectTestConfig::from_env();
    for test_path_stem in collect_test_path_stems(relative_test_dir) {
        let mut expects = ExpectInstance::extract_from_file(test_path_stem)
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
    #[error("io error: {e} for path {path:?}")]
    IO { e: std::io::Error, path: PathBuf },
    #[error("serde json")]
    SerdeJson(#[from] serde_json::error::Error),
}

type DesIoResult<T> = Result<T, DesIoError>;
