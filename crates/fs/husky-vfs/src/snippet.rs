use crate::*;

#[salsa::input(db = VfsDb, jar = VfsJar)]
pub struct Snippet {
    #[return_ref]
    pub data: String,
}
