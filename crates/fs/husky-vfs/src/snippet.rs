use crate::*;

#[salsa::input(db = VfsDb, jar = VfsJar)]
pub struct Snippet {
    pub ident: Ident,
    #[return_ref]
    pub data: String,
}
