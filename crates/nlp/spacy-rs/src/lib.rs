pub mod constituent_parsing;
pub mod token;

use self::token::*;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

pub fn python_src_dir(python_lib_dir: &Path) -> PathBuf {
    python_lib_dir.join("spacy-rs")
}
