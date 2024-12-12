pub mod constituent_parsing;
pub mod token;

use pyo3::prelude::*;
use std::path::PathBuf;

fn python_src_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("python")
}
