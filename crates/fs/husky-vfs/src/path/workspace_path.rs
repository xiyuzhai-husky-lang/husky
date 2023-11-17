use super::*;

#[salsa::interned(jar = VfsJar, db = VfsDb, constructor = new_inner)]
pub struct WorkspacePath {}
