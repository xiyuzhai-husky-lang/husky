// use timed_salsa::jar::Jar;

use std::path::PathBuf;

#[derive(Debug)]
pub struct HuskyFile {
    path: PathBufItd,
    content: String,
}

#[timed_salsa::interned]
pub struct PathBufItd {
    raw: PathBuf,
}

// todo: different Durabilities
#[timed_salsa::input]
pub struct LibraryFile {
    #[return_ref]
    inner: HuskyFile,
}

#[timed_salsa::input]
pub struct PublishedFile {
    #[return_ref]
    inner: HuskyFile,
}

#[timed_salsa::input]
pub struct UserFile {
    #[return_ref]
    inner: HuskyFile,
}

#[timed_salsa::jar(db = Db)]
pub struct Jar(PathBufItd, LibraryFile, PublishedFile, UserFile);

pub trait Db: timed_salsa::DbWithJar<Jar> {}
