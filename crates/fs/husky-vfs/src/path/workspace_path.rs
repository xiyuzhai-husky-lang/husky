use super::*;

#[salsa::interned(constructor = new_inner)]
pub struct WorkspacePath {}
