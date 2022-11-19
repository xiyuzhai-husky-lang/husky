// use timed_salsa::jar::Jar;

mod file;

use file::*;
use std::path::PathBuf;

#[timed_salsa::interned]
pub struct PathBufItd {
    raw: PathBuf,
}

#[timed_salsa::jar(db = Db)]
pub struct Jar(PathBufItd, HuskyFile);

pub trait Db: timed_salsa::DbWithJar<Jar> {}
