pub mod constituent;

use std::path::PathBuf;

fn python_src_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("python")
}
