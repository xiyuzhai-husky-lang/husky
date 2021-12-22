mod error;
mod once;
mod path_utils;
pub mod print;
pub mod show;
mod templates;
pub mod todo;

pub use error::*;
pub use once::do_once;
pub use path_utils::*;
pub use show::*;
pub use std::cell::Cell;
pub use std::collections::HashMap;
pub use std::fmt;
pub use std::fmt::Debug;
pub use std::fmt::Formatter;
pub use std::io;
pub use std::path::{Path, PathBuf};
pub use templates::*;
pub use todo::*;

pub mod test_utils {
    // import this function to tests module so that p! could be used
    pub fn assert_test_env() {}
}

pub type Range = std::ops::Range<usize>;
